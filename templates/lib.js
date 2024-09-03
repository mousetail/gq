const add = (a, b) => {
    if (a == null) {
        return null
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
        return `${a}${b}`
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
            if (eq(copy.slice(i, b.length), b)) {
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
            if (eq(a.slice(i, b.length), b)) {
                copy.push(a.slice(last_group, i - last_group));
                last_group = i + b.length;
                i = last_group - 1;
            }
        }
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
        const copy = [a];
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