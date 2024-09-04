use template_macros::{fragment, half_fragment};

use crate::language::builtin::{BracketHandler, Builtin, MultiOutputBehavior, OutputHandler};

pub const BUILTINS: &'static [Builtin] = &[
    Builtin {
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
                    behavior: MultiOutputBehavior::FlattenAll,
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
        token: 'r',
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
];
