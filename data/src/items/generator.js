
const input = `<omitted>`;

const inputFoods = `<omitted>`.replace(/stew\(([0-9]+)\)/g, `(new FoodProperties.Builder()).nutrition($1).saturationMod(0.6F).build()`);

const lines = input.split("\n");
const foodLines = inputFoods.split("\n");

String.prototype.appendIfPresent = function(other, otherwise='') {
    if (this.length > 0) {
        return this + other;
    } else {
        return this + otherwise;
    }
}

// escapes not handled
const parseCommaList = list => {
    const output = [];
    let parenDepth = 0;
    let inString = false;
    let lastSplit = 0;
    for (let i = 0; i < list.length; ++i) {
        const c = list[i];
        if (!inString && c == '(') {
            parenDepth++;
        } else if (!inString && c == ')') {
            parenDepth--;
        } else if (c == '"') {
            inString = !inString;
        } else if (parenDepth == 0 && c == ',') {
            output.push(list.slice(lastSplit, i).trim());
            lastSplit = i + 1;
        }
    }
    if (lastSplit != list.length) {
        output.push(list.slice(lastSplit, list.length).trim());
    }
    return output;
}

const propertize = properties => properties.split(').').map(x => x.startsWith('.') ? x.slice(1) : x).filter(x => x.length > 0).map(x => x.endsWith(')') ? x : x + ')');

const foods = {};

for (const line of foodLines) {
    const [_, name, propertiesRaw] = line.match(/([A-Z_0-9]+) = \(new FoodProperties\.Builder\(\)\)(.*?);/);
    const properties = propertize(propertiesRaw);
    let nutrition = 0;
    let saturationMod = 0;
    let isMeat = false;
    let alwaysEat = false;
    let fast = false;
    let effects = [];
    for (const property of properties) {
        if (property.startsWith('nutrition')) {
            nutrition = parseInt(property.match(/nutrition\(([0-9]+)\)/)[1]);
        } else if (property.startsWith('saturationMod')) {
            saturationMod = parseFloat(property.match(/saturationMod\(([0-9.]+)F?\)/)[1]).toFixed(2);
        } else if (property == 'meat()') {
            isMeat = true;
        } else if (property == 'alwaysEat()') {
            alwaysEat = true;
        } else if (property == 'fast()') {
            fast = true;
        } else if (property.startsWith('effect')) {
            const [_, effectName, duration, amplifier, probability] = property.match(/effect\(new MobEffectInstance\(MobEffects.([A-Z0-9_]+), ([0-9]+), ([0-9]+)\), (.*?)\)/);
            effects.push({
                name: effectName,
                duration: parseInt(duration),
                amplifier: parseInt(amplifier),
                probability: parseFloat(probability).toFixed(2),
            });
        }
    }
    foods[name] = {
        nutrition,
        saturationMod,
        isMeat,
        alwaysEat,
        fast,
        effects,
    };
}

let i = -1;
const results = [];

for (const line of lines) {
    ++i;
    {
        const parsed = line.match(/([A-Z_0-9]+) = registerItem\((?:\(String\))?"([a-zA-Z_0-9]+)", new ([A-za-z0-9_]+)\((.*?)\(?new Item.Properties\(\)\)?(.*?)\)\);/);
        if (parsed != null) {
            const [_, name, namespace_name, class_name, other_args, properties] = parsed;
            results.push({ type: 'item', name, namespace_name, class_name, other_args, properties });
            //console.log(name, namespace_name, class_name, other_args, properties);
            continue;
        }
    }
    {
        const parsed = line.match(/([A-Z_0-9]+) = registerBlock\((?:\(BlockItem\))?\(?new ([A-za-z0-9_]+)\(Blocks\.([A-Z0-9_]+)(.*?), \(new Item.Properties\(\)\)(.*?)\)\);/);
        if (parsed != null) {
            const [_, name, class_name, block_name, other_args, properties] = parsed;
            // console.log(name, class_name, block_name, other_args, properties);
            results.push({ type: 'complex_block', name, class_name, block_name, other_args, properties });
            continue;
        }
    }
    {
        const parsed = line.match(/([A-Z_0-9]+) = registerBlock\(Blocks\.([A-Z0-9_]+)(.*?)\);/);
        if (parsed != null) {
            const [_, name, block_name, other_args] = parsed;
            // console.log(name, block_name, other_args);
            results.push({ type: 'simple_block', name, block_name, other_args });
            continue;
        }
    }
    throw line;
}
for (const result of results) {
    if (result.type == 'simple_block') {
       console.log(`    lazy_static! { pub static ref ${result.name}: Item = { Arc::new(ItemT::from(blocks::${result.block_name}.clone())) }; }`);
    } else if (result.type == 'complex_block') {
        const properties = propertize(result.properties);
        let maxStackSize = 64;
        for (const arg of properties) {
            if (arg.startsWith('stacksTo')) {
                maxStackSize = parseInt(arg.match(/stacksTo\(([0-9]+)\)/)[1]);
            }
        }
        console.log(`    lazy_static! { pub static ref ${result.name}: Item = { ItemT::from_block_stack_size(blocks::${result.block_name}.clone(), ${maxStackSize}) }; } // ${result.class_name}<${result.other_args}>`);
    } else if (result.type == 'item') {
        const properties = propertize(result.properties);
        let maxStackSize = 64;
        let maxDamage = 0;
        let craftRemainder = "None";
        let foodProperties = "None";
        for (const arg of properties) {
            if (arg.startsWith('stacksTo')) {
                maxStackSize = parseInt(arg.match(/stacksTo\(([0-9]+)\)/)[1]);
            } else if (arg.startsWith('durability')) {
                maxDamage = parseInt(arg.match(/durability\(([0-9]+)\)/)[1]);
                maxStackSize = 1;
            } else if (arg.startsWith('craftRemainder')) {
                craftRemainder = `Some(${arg.match(/craftRemainder\(([A-Z_0-9]+)\)/)[1]}.clone())`;
            } else if (arg.startsWith('food')) {
                const rawFood = arg.match(/food\(Foods.([A-Z_0-9]+)\)/)[1];
                const food = foods[rawFood];
                foodProperties = `Some(FoodProperties { nutrition: ${food.nutrition}, saturation_mod: ${food.saturationMod}, is_meat: ${food.isMeat}, can_always_eat: ${food.alwaysEat}, fast_food: ${food.fast}, effects: vec![${food.effects.map(effect => `(mob_effects::${effect.name}.instance(${effect.duration}, ${effect.amplifier}), ${effect.probability})`)}] })`
            }
        }

        console.log(`    lazy_static! { pub static ref ${result.name}: Item = { Arc::new(ItemT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0), max_stack_size: ${maxStackSize}, max_damage: ${maxDamage}, crafting_remaining_item: ${craftRemainder}, food_properties: ${foodProperties} }) }; } // ${result.class_name}<${result.other_args}>`);
    }
}

results.forEach(result => {
    const output = `        registry.insert(${result.namespace_name != null ? `"minecraft:${result.namespace_name}"` : `&blocks::${result.name}.registry_name.clone()`}, *${result.name});`;
    console.log(output);
});
