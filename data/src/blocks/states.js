const blockStateReport = require('./block_state_report.json');

const uniqueNames = {};

const report = Object.fromEntries(Object.entries(blockStateReport).map(([block, { properties }]) => [block, properties]));

const classifyProperty = (name, property) => {
    const processed = property.slice().sort().join(',');
    if (processed == 'false,true') {
        return 'bool';
    } else if (processed == 'east,north,south,west') {
        return 'facing_horizontal';
    } else if (processed == 'down,east,north,south,up,west') {
        return 'facing';
    } else if (processed == 'lower,upper') {
        return 'flower';
    } else if (processed == 'bottom,double,top') {
        return 'slab';
    } else if (processed == 'bottom,top') {
        return 'stairs';
    } else if (processed == 'left,right') {
        return 'hinge';
    } else if (processed == 'x,y,z') {
        return 'axis';
    } else if (processed == 'inner_left,inner_right,outer_left,outer_right,straight') {
        return 'stairs_shape';
    } else if (processed == 'foot,head') {
        return 'bed';
    } else if (processed.match(/^([0-9]+,?)*[0-9]+$/)) {
        return ['ordinal', property.slice().map(x => parseInt(x)).sort()[property.length - 1] + 1];
    }
    return 'unknown';
}

const camelify = name => {
    const split = name.split('_');
    return `${split.map(x => `${x[0].toUpperCase()}${x.slice(1)}`).join('')}`;
};

const enums = {};
const results = {};
const wrap_keyword = x => x == 'type' ? 'type_' : x;
for (const [block, properties] of Object.entries(report)) {
    results[block] = {};
    for (const name in properties) {
        const classified = classifyProperty(name, properties[name]);
        let result = null;
        if (Array.isArray(classified)) {
            const [name, argument] = classified;
            if (name == 'ordinal') {
                result = `IntProperty { maximum: ${argument} }`;
            } else {
                throw new Error('unexpected name in argumented classification: ' + name);
            }
            continue;
        }
        if (classified == 'bool') {
            result = 'bool';
        } else {
            let preName = name;
            const property = properties[name];
            if (classified == 'facing') {
                preName = 'facing';
            } else if (classified == 'facing_horizontal') {
                preName = 'facing_horizontal'
            } else if (classified == 'slab') {
                preName = 'slab_type'
            } else if (name == 'facing') {
                preName = 'facing_downable';
            } else if (name == 'half') {
                preName = classified;
            } else if (name == 'shape') {
                preName = property[0] == 'north_south' ? (property[property.length - 1] == 'north_east' ? 'rail_shape' : 'straight_rail_shape') : 'stair_shape';
            } else if (name == 'type') {
                preName = property[0] == 'single' ? 'chest_type' : 'piston_type';
            } else if (name == 'axis') {
                preName = property[1] == 'y' ? 'axis' : 'horizontal_axis';
            } else if (name == 'mode') {
                preName = property[0] == 'compare' ? 'comparator_mode' : 'structure_mode'
            }
            const camel = camelify(preName);
            result = camel;
            const enumStr =
`#[cenum]
#[derive(Copy)]
pub enum ${camel} {
    ${property.map(camelify).join(',\n    ')}
}

impl Default for ${camel} {
    fn default() -> ${camel} { ${camel}::${camelify(property[0])} }
}

impl BlockStateProperty<${camel}> for ${camel} {
    fn all(&self) -> Vec<${camel}> {
        vec![${property.map(p => `${camel}::${camelify(p)}`).join(', ')}]
    }

    fn state_from(&self, value: &str) -> Option<${camel}> {
        match value {
            ${property.map(p => `"${p}" => Some(${camel}::${camelify(p)})`).join(',\n            ')},
            _ => None,
        }
    }
}

`;
            if (enums[camel] == null) {
                enums[camel] = enumStr;
            } else if (enums[camel] != enumStr) {
                throw new Error(`enum mismatch: ${camel}\n${enumStr}\n${enums[camel]}`);
            }
        }
        results[block][name] = result;
    }
}
console.log(Object.values(enums).join(''));

function recurseAll(name, data, waiting = []) {
    const item = data[0];
    const new_waiting = [...waiting, item];
    const out = `
    for ${item}_i in self.${wrap_keyword(item)}.all() {
        ${data.length > 1 ? recurseAll(name, data.slice(1), new_waiting) : `out.push(${name} { ${new_waiting.map(x => `${wrap_keyword(x)}: ${x}_i`).join(',')} })`}
    }
    `;
    return out;
}

for (const [block, data] of Object.entries(results)) {
    const camel_name = `BlockState${camelify(block.split(':')[1])}`;
    if (Object.entries(data).length > 0) {
        const block_name = block.split(':')[1].toUpperCase();
        const struct =
`#[derive(Default, Clone, Debug)]
pub struct ${camel_name} {
    ${Object.entries(data).map(([name, type]) => `pub ${wrap_keyword(name)}: ${type},`).join('\n    ')}
}

impl BlockStateContainerImpl for ${camel_name} {}

impl BlockStateContainer for ${camel_name} {
    fn all(&self) -> Vec<Self> {
        let mut out: Vec<Self> = vec![];
        ${recurseAll(camel_name, Object.keys(data))}
        return out;
    }

    fn state_from(&self, value: &HashMap<String, String>) -> Option<Self> {
        Some(${camel_name} { ${Object.keys(data).map((name) => `${wrap_keyword(name)}: self.${wrap_keyword(name)}.state_from(value.get("${name}")?)?`).join(',')} })
    }
}

`;
        console.log(struct);
    } else {
        console.log(`pub type ${camel_name} = BlockStateNone;\n`);
    }
}

//console.log(JSON.stringify(results, null, 2));
