use template_macros::{fragment, half_fragment};
use template_types::ProgramFragment;

use crate::builtin::{BracketHandler, Builtin};

pub const BUILTINS: &'static [Builtin] = &[
    Builtin {
        token: ',',
        template: fragment!(
            "
            const {wrapper:local} = () => {{
                {inner}
            "
        ),
        bracket_handlers: &[
            BracketHandler {
                output_handler: Some(half_fragment!(
                    "
                    {output:local}({value:in});
                    "
                )),
                fragment: ProgramFragment {
                    arguments_popped: 0,
                    arguments_pushed: 0,
                    init_tokens: &[],
                    destruct_tokens: &[],
                },
            },
            BracketHandler {
                output_handler: Some(half_fragment!(
                    "
                    {output:local}({value:in});
                    "
                )),
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
            },
        ],
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
            output_handler: Some(half_fragment!(
                "
                {arr:local}.push({value:in});
            "
            )),
            fragment: fragment!(
                "
                const {out:out} = {arr:local};
                {inner}
                "
            ),
        }],
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
];
