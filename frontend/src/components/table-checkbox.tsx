"use client"

import { useState } from "react";
import { Checkbox } from "./ui/checkbox";
import { toast } from "sonner";
import { useRouter } from "next/navigation";

interface TableCheckboxProps {
    id: number,
    hired: boolean
}

export default function TableCheckBox({id, hired}: TableCheckboxProps) {
    const router = useRouter();

    const [checked, setChecked] = useState(hired);

    async function toggleChecked() {
        setChecked(!checked);

        const response = await fetch(`/api/resume/hire?id=${id}`, {
            method: "PATCH"
        })

        if(response.status == 401) {
            toast.error("Login expirado");

            router.push("login")
        }

        if(!response.ok) {
            toast.error("Falha ao atualizar campo")
            setChecked(checked);

            return;
        }
    }

    return <Checkbox checked={checked} onCheckedChange={toggleChecked}/>
}