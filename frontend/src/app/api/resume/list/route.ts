import { cookies } from "next/headers";
import { NextRequest, NextResponse } from "next/server";

const backend_url = process.env.BACKEND_URL;

export async function GET(req: NextRequest) {
    const token = (await cookies()).get("token")?.value;

    const searchParams = req.nextUrl.searchParams;
    const page = searchParams.get("page") ?? 1;

    const response = await fetch(`${backend_url}/resume/list/${page}?search=&id_sector=0&uninterviewed=false`, {
        method: "GET",
        headers: {
            "Authorization": `Bearer ${token}`
        }
    });

    if(![200, 400, 401].includes(response.status)) {
        return NextResponse.json({ error: "Erro desconhecido" }, { status: 500 })
    }

    if(response.status == 401) {
        return NextResponse.json({}, {status: 401})
    }

    if(!response.ok) {
        return NextResponse.json({}, {status: 400})
    }

    return NextResponse.json({ resumes: (await response.json()).resumes})
}