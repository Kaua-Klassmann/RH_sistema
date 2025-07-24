"use client"

import { useRouter } from "next/navigation";
import { Button } from "./ui/button";

export default function LogoffButton() {
    const router = useRouter();

    async function logoff() {
        await fetch("/api/logoff", {
            method: "POST"
        })

        router.push("/login");
    }

    return <Button className="w-1/2" variant={"destructive"} onClick={logoff}>Sair</Button>
}