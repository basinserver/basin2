const input = `<omitted>`;

const lines = input.split("\n");

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

let i = -1;
const results = [];

const ignoredEnums = ['MaterialColor', 'DyeColor'];

for (const line of lines) {
    ++i;
    const parsed = line.match(/([A-Z_]+) = register\("([a-zA-Z_0-9]+)", new ([A-za-z0-9_]+)\((.*?)\)\);/);
    const [_,  name, namespace_name, class_name, args_raw] = parsed;
    const args = parseCommaList(args_raw);
    let properties = null;
    const otherArgs = [];
    for (const arg of args) {
        if (arg.startsWith('Block.Properties.of')) {
            const [constructor, material] = arg.match(/Block\.Properties\.of\(Material\.([a-zA-Z0-9_]+).*?\)/);
            const props = arg.slice(constructor.length).split(').').map(x => x.startsWith('.') ? x.slice(1) : x).filter(x => x.length > 0).map(x => x.endsWith(')') ? x : x + ')');
            const rustArgs = props.map(property => {
                if (property == 'noCollission()') {
                    return 'has_collision: false, can_occlude: false';
                } else if (property == 'noOcclusion()') {
                    return 'can_occlude: false';
                } else if (property.startsWith('friction')) {
                    return `friction: ${parseFloat(property.match(/friction\((.*?)\)/)[1]).toFixed(2)}`;
                } else if (property.startsWith('speedFactor')) {
                    return `speed_factor: ${parseFloat(property.match(/speedFactor\((.*?)\)/)[1]).toFixed(2)}`;
                } else if (property.startsWith('jumpFactor')) {
                    return `jump_factor: ${parseFloat(property.match(/jumpFactor\((.*?)\)/)[1]).toFixed(2)}`;
                } else if (property.startsWith('lightLevel')) {
                    return `light_emission: ${property.match(/lightLevel\((.*?)\)/)[1]}`;
                } else if (property.startsWith('strength')) {
                    const match = property.match(/strength\((.*?)(?:, (.*?))?\)/);
                    const resistance = parseFloat(match[2] || match[1]);
                    return `destroy_time: ${parseFloat(match[1]).toFixed(2)}, explosion_resistance: ${(resistance > 0 ? resistance : 0.0).toFixed(2)}`;
                } else if (property == 'instabreak()') {
                    return ''; // is default
                } else if (property == 'randomTicks()') {
                    return 'is_ticking: true';
                } else if (property == 'dynamicShape()') {
                    return 'dynamic_shape: true';
                } else if (property == 'noDrops()') {
                    return ''; // is default
                } else if (property.startsWith('dropsLike')) {
                    return `drops: ${property.match(/dropsLike\((.*?)\)/)[1]}.drops.clone()`;
                } else if (property.startsWith('sound')) {
                    return ''; // ignored
                } else throw property;
            }).filter(x => x.length > 0).join(', ').split(', ');
            properties = { material, rustArgs };
        } else if (arg.startsWith('Block.Properties.copy')) {
            properties = arg.match(/Block.Properties.copy\((.*?)\)/)[1];
        } else if (ignoredEnums.find(x => arg.startsWith(x))) {
            // ignored
        } else {
            otherArgs.push(arg);
        }
        results[i] = { name, namespace_name, class_name, properties, otherArgs };
    }
}
for (const i in results) {
    if (typeof results[i].properties == 'string') { // is a copy
        results[i].properties = results.find(x => x.name == results[i].properties).properties;
    }
}
results.forEach(result => {
    const output = `    lazy_static! { pub static ref ${result.name}: Block = { Arc::new(BlockT { material: materials::${result.properties.material}.clone(), ${result.properties.rustArgs.join(', ').appendIfPresent(', ', ' ')}..Default::default() }) }; }`;
    console.log(output);
});

results.forEach(result => {
    const output = `        registry.insert("minecraft:${result.namespace_name}", ${result.name}.clone());`;
    console.log(output);
});
