import { cookies } from "next/headers";
import { NextResponse } from "next/server";

const backend_url = process.env.BACKEND_URL;

export async function GET() {
    const token = (await cookies()).get("token")?.value;

    const response = await fetch(`${backend_url}/resume/pages`, {
        method: "GET",
        headers: {
            "Authorization": `Bearer ${token}`
        }
    })

    if(response.status == 401) {
        return NextResponse.json({}, { status: 401 })
    }

    if(response.status == 500) {
        return NextResponse.json({}, { status: 500 })
    }

    const datas = await response.json();

    return NextResponse.json({ pages: datas.pages })
}