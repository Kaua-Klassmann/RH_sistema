import { cookies } from "next/headers";
import { NextResponse } from "next/server";

const backend_url = process.env.BACKEND_URL;

export async function GET() {
    const token = (await cookies()).get("token")?.value;

    const response = await fetch(`${backend_url}/sector/list`, {
        method: "GET",
        headers: {
            "Authorization": `Bearer ${token}`
        }
    });

    if(!response.ok) {
        return NextResponse.json({}, { status: response.status })
    }

    const data = await response.json();

    return NextResponse.json({ setores: data.setores })
}