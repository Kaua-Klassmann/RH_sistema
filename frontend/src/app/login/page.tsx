"use client"

import { useState } from "react"
import { useRouter } from "next/navigation"

import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card"
import { toast } from "sonner"

export default function Login() {
    const router = useRouter();

    const [user, setUser] = useState("");
    const [password, setPassword] = useState("");

    async function handleSubmit(e: React.FormEvent) {
        e.preventDefault();

        if(user.length < 3 || password.length < 6) {
            toast.error("Campos inválidos")
            return;
        } 

        const response = await fetch("/api/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({user, password})
        })

        if (response.ok) {
            router.push("/resume/list");
        } else {
            const data = await response.json();

            toast.error(data.error)
        }
    }

    return (
        <div className="flex min-h-svh w-full items-center justify-center p-6">
            <div className="w-full max-w-sm">
                <div className={cn("flex flex-col gap-6")}>
                    <Card>
                        <CardHeader>
                            <CardTitle className="text-center text-3xl">Login</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <form onSubmit={handleSubmit}>
                                <div className="flex flex-col gap-6">
                                    <div className="grid gap-3">
                                        <Label htmlFor="user">Usuário</Label>
                                        <Input
                                            id="user"
                                            type="text"
                                            value={user}
                                            onChange={e => setUser(e.target.value)}
                                            placeholder="usuário123"
                                            required
                                        />
                                    </div>
                                    <div className="grid gap-3">
                                        <Label htmlFor="password">Senha</Label>
                                        <Input
                                            id="password"
                                            type="password"
                                            value={password}
                                            onChange={e => setPassword(e.target.value)}
                                            placeholder="senha"
                                            required
                                        />
                                    </div>
                                    <div>
                                        <Button type="submit" className="w-full">
                                            Login
                                        </Button>
                                    </div>
                                </div>
                            </form>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </div>
    )
}