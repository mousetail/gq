use template_macros::{fragment, half_fragment};

use crate::language::builtin::{BracketHandler, Builtin, MultiOutputBehavior, OutputHandler};

pub const BUILTINS: &'static [Builtin] = &[
    Builtin {
        name: "bracket",
        description: "no-op group",
        token: '(',
        template: fragment!(
            "
            const {wrapper:local} = () => {{
                {inner}
            "
        ),
        bracket_handlers: &[BracketHandler {
            output_handler: Some(OutputHandler {
                fragment: half_fragment!(
                    "
                        {output:local}({value:in});
                    "
                ),
                behavior: MultiOutputBehavior::FlattenAll,
            }),
            fragment: fragment!(
                "
                        //
                    }};
                    const {output:local} = ({out_var:out})=>{{
                        {inner}
                    }}

                    {wrapper:local}();
                "
            ),
        }],
    },
    Builtin {
        name: "List wrap",
        description: "Create a list/array from the items generated by the enclosed generator.",
        token: '[',
        template: fragment!(
            "
            const {arr:local} = [];
            { inner }
        "
        ),
        bracket_handlers: &[BracketHandler {
            output_handler: Some(OutputHandler {
                fragment: half_fragment!(
                    "
                        {arr:local}.push({value:in});
                        "
                ),
                behavior: MultiOutputBehavior::FlattenAll,
            }),
            fragment: fragment!(
                "
                    const {out:out} = {arr:local};
                    {inner}
                    "
            ),
        }],
    },
    Builtin {
        name: "If",
        description: "Pops one argument, executes the first block if true and the second if false",
        token: '?',
        template: fragment!(
            "
            const { condition:local } = to_bool({condition_var:in});

            const {if_true:local} = ()=>{{
                {inner}
            "
        ),
        bracket_handlers: &[
            BracketHandler {
                output_handler: None,
                fragment: fragment!(
                    "
                    //
                }}
                const {if_false:local} = ()=>{{
                    {inner}
                }}
                "
                ),
            },
            BracketHandler {
                output_handler: Some(OutputHandler {
                    fragment: half_fragment!(
                        "
                        {inner:local}({value:in})
                        "
                    ),
                    behavior: MultiOutputBehavior::Variadic,
                }),
                fragment: fragment!(
                    "
                    const {inner:local} = ({value:out}) => {{
                        {inner}
                    }}

                    if ({condition:local}) {{
                        {if_true:local}();
                    }} else {{
                        {if_false:local}();
                    }}
                "
                ),
            },
        ],
    },
    Builtin {
        name: "Reduce",
        description: "Pops an accumulator, then applies the second group to the items from the first group. Outputs the final accumulator.",
        token: 'R',
        template: fragment!(
            "
            let {accumulator:local} = {initial:in};
            
            const {inner:local} = () => {{
                {inner}
            
            "
        ),
        bracket_handlers: &[
            BracketHandler {
                output_handler: Some(OutputHandler {
                    fragment: half_fragment!(
                        "
                            {next:local}({accumulator:local}, {value:in})
                            "
                    ),
                    behavior: MultiOutputBehavior::FlattenAll,
                }),
                fragment: fragment!(
                    "
                    //
                }}

                const {next:local} = ({accumulator:out}, {value:out}) => {{
                    {inner}
                }}
                "
                ),
            },
            BracketHandler {
                output_handler: Some(OutputHandler {
                    fragment: half_fragment!(
                        "
                        {accumulator:local} = {value:in};
                        "
                    ),
                    behavior: MultiOutputBehavior::OnlyFirst,
                }),
                fragment: fragment!(
                    "
                {inner:local}();

                {out:out} = {accumulator:local};
                "
                ),
            },
        ],
    },
    Builtin {
        name: "range",
        description: "Outputs the numbers from 0 to N exclusive",
        token: 'r', // I might be able to make array unwrap on a number do range
        template: fragment!(
            "
            for (let {i:out}=0;{i:out}<{max:in};{i:out}++) {{
                {inner}
            }}
            "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        name: "plus",
        description: "adds",
        token: '+',
        template: fragment!(
            "
            const {out:out} = add({op1:in}, {op2:in});
            { inner }
        "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        name: "sub",
        description: "subtracts, or removes items from an array or string",
        token: '-',
        template: fragment!(
            "
            const {out:out} = sub({op1:in}, {op2:in});
            { inner }
        "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        name: "times",
        description: "multiplies",
        token: '*',
        template: fragment!(
            "
            const {out:out} = mul({op1:in}, {op2:in});
            { inner }
        "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        name: "div",
        description: "divides",
        token: '/',
        template: fragment!(
            "
            const {out:out} = div({op1:in}, {op2:in});
            { inner }
            "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        name: "equals",
        description: "performs a deep equals check",
        token: '=',
        template: fragment!(
            "
            const {out:out} = eq({op1:in}, {op2:in});
            { inner }
            "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        name: "Unwrap",
        description: "Outputs each value from an array seperately",
        token: 'u',
        template: fragment!(
            "
            for (const {out:out} of iter({op1:in})){{
                { inner }
            }}
            "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        name: "Dup",
        description: "Pops a value from the stack, then pushes it twice",
        token: ':',
        template: fragment!(
            "
            const {out1:out} = {value:in}, {out2:out} = {value:in};
            {inner}
            "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        name: "Swap",
        description: "Pops two values from the stack then pushes them in reverse order",
        token: '$',
        template: fragment!(
            "
            const [{out1:out},{out2:out}] = [{value1:in},{value2:in}];
            {inner}
            "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        name: "Over",
        description: "Pops 2 values from the stack, pushes them back in order, then pushes the bottom most one again",
        token: '@',
        template: fragment!(
            "
            // {value1:in}
            const [{out1:out},{out2:out},{out3:out}] = [{value2:in},{value1:in},{value2:in}];
            {inner}
            "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        name: "List Sum",
        description: "Pops an array then outputs the sum of each element",
        token: 's',
        template: fragment!(
            "
            const {out:out} = iter({in:in}).reduce(add, null);
            {inner}
            "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        name: "Generator Sum",
        description: "Sums the outputs of a generator",
        token: 'S',
        template: fragment!(
            "
            let {accumulator:local} = null;
            {inner}
            "
        ),
        bracket_handlers: &[BracketHandler {
            output_handler: Some(OutputHandler {
                behavior: MultiOutputBehavior::FlattenAll,
                fragment: half_fragment!(
                    "
                            {accumulator:local} = add({accumulator:local}, {value:in});
                            "
                ),
            }),
            fragment: fragment!(
                "
                    const {out:out} = {accumulator:local};
                "
            ),
        }],
    },
];
