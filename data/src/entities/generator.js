const input = `<omitted>`;

const lines = input.split("\n");

String.prototype.appendIfPresent = function(other, otherwise='') {
    if (this.length > 0) {
        return this + other;
    } else {
        return this + otherwise;
    }
}

for (const line of lines) {
    const run = (name, registry_name, class_name, args_raw) => {
        const args = args_raw.split(').').map(x => x.startsWith('.') ? x.slice(1) : x).filter(x => x.length > 0).map(x => x.endsWith(')') ? x : x + ')');
        const rustArgs = args.map(arg => {
            if (arg.startsWith('sized(')) {
                let [_, width, height] = arg.match(/sized\((.*?), (.*?)\)/);
                return `dimensions: (${parseFloat(width).toFixed(2)}, ${parseFloat(height).toFixed(2)})`;
            } else if (arg == 'noSummon()') {
                return 'should_summon: false';
            } else if (arg == 'noSave()') {
                return 'should_save: false';
            } else if (arg == 'fireImmune()') {
                return 'fire_immune: true';
            } else if (arg == 'canSpawnFarFromPlayer()') {
                return 'can_spawn_far_away: true';
            } else throw arg;
        });
        const output = `    lazy_static! { pub static ref ${name}: EntityType = { Arc::new(EntityTypeT { class_name: ${class_name}, ${rustArgs.join(', ').appendIfPresent(', ', ' ')}..Default::default() }) }; }`
        console.log(output); 
        const output2 = `        registry.insert("${registry_name}", ${name}.clone());`;
        console.error(output2); 
    };
    const parsed = line.match(/([A-Z_]+) = register\("([^\"]*)", EntityType\.Builder\.of\(([^:]+)::new, .*?\)((?:\.[a-zA-Z0-9_]+\(.*?\))*?)\);/);
    if (parsed == null) {
        const parsedNothing = line.match(/([A-Z_]+) = register\("([^\"]*)", EntityType\.Builder\.createNothing\(.*?\)((?:\.[a-zA-Z0-9_]+\(.*?\))*?)\);/);
        const [_,  name, registry_name, args_raw] = parsedNothing;
        run(name, registry_name, `None`, args_raw);

        continue;
    } else {
        const [_,  name, registry_name, class_name, args_raw] = parsed;
        run(name, registry_name, `Some("${class_name}".to_string())`, args_raw);
    }


}