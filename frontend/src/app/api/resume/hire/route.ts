import { cookies } from "next/headers";
import { NextRequest, NextResponse } from "next/server";

const backend_url = process.env.BACKEND_URL;

export async function PATCH(req: NextRequest){
    const token = (await cookies()).get("token")?.value;

    if(!token) {
        return NextResponse.json({}, { status: 401 })
    }

    const searchParams = req.nextUrl.searchParams;
    const id = searchParams.get("id");

    if(!id) {
        return NextResponse.json({}, { status: 400 })
    }

    const response = await fetch(`${backend_url}/resume/${id}/hire`, {
        method: "PATCH",
        headers: {
          "Authorization": `Bearer ${token}`
        }
    })

    if(!response.ok) {
        return NextResponse.json({}, { status: 400 })
    }

    return NextResponse.json({})
}