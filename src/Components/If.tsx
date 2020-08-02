import React, { PropsWithChildren } from "react";

interface IfProps {
    condition: boolean;
}

export default function If(props: PropsWithChildren<IfProps>) {
    if (props.condition === true) {
        return <>{props.children}</>;
    }

    return <></>;
}
