use template_macros::{fragment, half_fragment};

use crate::builtin::{BracketHandler, Builtin, MultiOutputBehavior, OutputHandler};

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
        bracket_handlers: &[
            BracketHandler {
                output_handler: None,
                fragment: fragment!(""),
            },
            BracketHandler {
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
            },
        ],
    },
    Builtin {
        token: '?',
        template: fragment!(
            "
            const { condition:local } = {condition_var:in};

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
            const {out:out} = {op1:in} + {op2:in};
            { inner }
        "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        token: '1',
        template: fragment!(
            "
            const {out:out} = 1;
            "
        ),
        bracket_handlers: &[],
    },
    Builtin {
        token: '5',
        template: fragment!(
            "
            const {out:out} = 5;
            "
        ),
        bracket_handlers: &[],
    },
];
