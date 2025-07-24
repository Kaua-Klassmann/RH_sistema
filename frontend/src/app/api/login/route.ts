import { NextRequest, NextResponse } from "next/server";
import { cookies } from "next/headers";

const backend_url = process.env.BACKEND_URL;

export async function POST(req: NextRequest) {
    const { user, password } = await req.json();

    if(user.length < 3 || password.length < 6) {
        return NextResponse.json({ error: "Estrutura inválida" }, { status: 422 })
    } 

    const response = await fetch(`${backend_url}/user/login`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            user,
            password
        })
    });

    if(![200, 400].includes(response.status)) {
        return NextResponse.json({ error: "Erro desconhecido" }, { status: 500 })
    }

    const data = await response.json();

    if(!response.ok) {
        return NextResponse.json({ error: data.error }, { status: 400 })
    }

    (await cookies()).set("token", data.token, {
        httpOnly: true,
        path: "/",
        maxAge: 60 * 60 * 24
    });

    return NextResponse.json({ ok: true })
}