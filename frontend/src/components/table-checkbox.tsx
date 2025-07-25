"use client"

import { useState } from "react";
import { Checkbox } from "./ui/checkbox";

interface TableCheckboxProps {
    id: number,
    hired: boolean
}

export default function TableCheckBox({id, hired}: TableCheckboxProps) {
    const [checked, setChecked] = useState(hired);

    return <Checkbox checked={checked} onCheckedChange={() => setChecked(!checked)}/>
}