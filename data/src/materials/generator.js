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
    const parsed = line.match(/([A-Z_]+) = \(new Material.Builder\(MaterialColor.[A-Z_]+\)\)((?:\.[a-zA-Z0-9_]+\(.*?\))*?)\.build\(\);/);
    const [_,  name, args_raw] = parsed;
    const args = args_raw.split(').').map(x => x.startsWith('.') ? x.slice(1) : x).filter(x => x.length > 0).map(x => x.endsWith(')') ? x : x + ')');
    const rustArgs = args.map(arg => {
        if (arg == 'liquid()') {
            return 'liquid: true';
        } else if (arg == 'nonSolid()') {
            return 'solid: false';
        } else if (arg == 'noCollider()') {
            return 'blocks_motion: false';
        } else if (arg == 'notSolidBlocking()') {
            return 'solid_blocking: false';
        } else if (arg == 'notAlwaysDestroyable()') {
            return 'always_destroyable: false';
        } else if (arg == 'flammable()') {
            return 'flammable: true';
        } else if (arg == 'replaceable()') {
            return 'replaceable: false';
        } else if (arg == 'destroyOnPush()') {
            return 'push_reaction: PushReaction::Destroy';
        } else if (arg == 'notPushable()') {
            return 'push_reaction: PushReaction::Block';
        } else throw arg;
    });
    const output = `    pub static ref ${name}: Material = { Arc::new(MaterialT { ${rustArgs.join(', ').appendIfPresent(', ', ' ')}..Default::default() }) };`
    console.log(output);
}

