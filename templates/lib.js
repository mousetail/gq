const add = (a, b) => {
    if (a == null) {
        return b
    } else if (b == null) {
        return a
    } else if (typeof a == 'number' && typeof b == 'number' ||
        typeof a == 'string' && typeof b == 'string'
    ) {
        return a + b
    }
    else if (typeof a == 'object' && typeof b == 'object') {
        return [...a, ...b]
    } else if (typeof a == 'string') {
        return `${a}${JSON.stringify(b)}`
    } else if (typeof a == 'object') {
        return [...a, b]
    } else {
        return null;
    }
}

const sub = (a, b) => {
    if (a == null) {
        return null
    } else if (b == null) {
        return a
    } else if (typeof a == 'number' && typeof b == 'number') {
        return a - b
    } else if (typeof a == 'string' && typeof b == 'string') {
        return a.replaceAll(b, '')
    } else if (typeof a == 'object' && typeof b == 'object') {
        const copy = [...a];
        for (let i = 0; i < copy.length; i++) {
            if (eq(copy.slice(i, i + b.length), b)) {
                copy.splice(i, b.length);
                i -= 1;
            }
        }
        return copy;
    } else if (typeof a == 'string') {
        return sub(a, `${b}`)
    } else if (typeof a == 'object') {
        return a.filter((d) => !eq(d, b))
    } else {
        return null;
    }
}

const eq = (a, b) => {
    if (a == b) {
        return 1
    }
    if (typeof a == 'object' && typeof b == 'object' && a.length == b.length) {
        for (let i = 0; i < a.length; i++) {
            if (!eq(a[i], b[i])) {
                return 0;
            }
        }
        return 1;
    }
    return 0;
}

const div = (a, b) => {
    if (a == null) {
        return null
    } else if (b == null) {
        return a
    } else if (typeof a == 'number' && typeof b == 'number') {
        return a / b
    } else if (typeof a == 'string' && typeof b == 'string') {
        return a.split(b)
    } else if (typeof a == 'object' && typeof b == 'object') {
        const copy = [];
        let last_group = 0;
        for (let i = 0; i < a.length; i++) {
            if (eq(a.slice(i, i + b.length), b)) {
                copy.push(a.slice(last_group, i - last_group));
                last_group = i + b.length;
                i = last_group - 1;
            }
        }
        copy.push(a.slice(last_group));
        return copy;
    } else if (typeof a == 'string') {
        return div(a, `${b}`)
    } else if (typeof a == 'object') {
        const copy = [];
        let last_group = 0;
        for (let i = 0; i < a.length; i++) {
            if (eq(a[i], b)) {
                copy.push(a.slice(last_group, i - last_group));
                last_group = i + 1;
            }
        }
        copy.push(a.slice(last_group));
        return copy;
    } else {
        return null;
    }
}

const mul = (a, b) => {
    if (a == null) {
        return null
    } else if (b == null) {
        return a
    } else if (typeof a == 'number' && typeof b == 'number') {
        return a * b
    } else if (typeof a == 'string' && typeof b == 'string') {
        return [...a].join(b)
    } else if (typeof a == 'object' && typeof b == 'object') {
        const copy = [];
        for (let i = 0; i < a.length; i++) {
            copy.push(a[i]);
            if (i < a.length - 1) {
                copy.push(...b)
            }
        }
        return copy;
    } else if (typeof a == 'string' && typeof b == 'number') {
        return a.repeat(b)
    } else if (typeof a == 'object' && typeof b == 'number') {
        const copy = []
        for (let i = 0; i < b; i++) {
            copy.push(...a)
        }
        return copy;
    } else if (typeof a == 'object') {
        const copy = [a];
        for (let i = 0; i < a.length; i++) {
            copy.push(a[i]);
            if (i < a.length - 1) {
                copy.push(b)
            }
        }
        return copy;
    } else {
        return null;
    }
}

const iter = (param) => {
    if (typeof param == 'number') {
        return [param]
    } else {
        return param
    }
}

const to_bool = (param) => {
    if (param == null) {
        return false;
    }
    if (typeof param == 'number') {
        return param > 0
    }
    return param.length > 0
}

const index = (arr, idx) => {
    let arr_iter = iter(arr);

    if (typeof idx == 'number') {
        idx = Math.floor(idx);
        if (idx >= 0 && idx < arr_iter.length) {
            return arr_iter[idx]
        } else if (idx < 0 && idx >= - arr_iter.length) {
            return arr_iter[arr_iter.length + idx];
        } else {
            return null;
        }
    }

    else if (typeof idx == 'object') {
        return idx.map(
            (k) => index(arr_iter, k)
        )
    }
}

const generator_index = (idx, callback) => {
    let floored_index = iter(idx).map((k) => Math.floor(k));
    let any_negavive = floored_index.some((k) => k < 0);

    if (any_negavive) {
        let output = [];

        callback((value) => {
            output.push(value);
            return false;
        });

        return index(output, idx);
    } else {
        let expected_outputs = new Map();
        let generator_index = 0;

        let expected_outputs_set = new Set(floored_index);

        callback((value) => {
            if (expected_outputs_set.has(generator_index)) {
                expected_outputs.set(generator_index, value);
            }

            generator_index += 1;

            if (expected_outputs.size == expected_outputs_set.size) {
                return true
            } else {
                return false
            }
        });

        return typeof idx == 'number' ? expected_outputs.get(floored_index[0]) : floored_index.map((d) => expected_outputs.get(d));
    }
}

const array_zip = (a, b) => {
    return new Array(Math.min(a.length, b.length)).fill(0).map((_e, i) => {
        return [a[i], b[i]];
    });
}

function* generator_zip(cb_1, cb_2) {
    let iterators = [cb_1, cb_2].map(i => i()[Symbol.iterator]());
    while (true) {
        let results = iterators.map(iter => iter.next())
        if (results.some(res => res.done)) return
        else yield results.flatMap(res => res.value)
    }
}

const flatten = (a, out) => {
    for (const value of a) {
        if (typeof value == 'object') {
            flatten(value, out)
        } else {
            out.push(value)
        }
    }
}

const flatten_floor = (a) => {
    if (a == null) {
        return null
    } if (typeof a == 'string') {
        return a
    } if (typeof a == 'number' || typeof a == 'boolean') {
        return Math.floor(a)
    } if (typeof a == 'undefined') {
        return null;
    } else {
        let out = [];
        flatten(a, out);
        return out;
    }
}

const mod = (a, b) => {
    if (b == null) {
        return a
    } else if (a == null) {
        return null
    }

    if (typeof a == 'number' && typeof b == 'number') {
        return a % b
    }

    // todo: Implement for arrays and strings
    console.warn("mod between non-numbers is not yet implemented");
    return null;
}

const decrement = (a) => {
    if (typeof a == 'number') {
        return a - 1;
    } else {
        return null;
    }
}