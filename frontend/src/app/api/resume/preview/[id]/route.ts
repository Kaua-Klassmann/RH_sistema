import { cookies } from "next/headers";
import { NextRequest, NextResponse } from "next/server";

const backend_url = process.env.BACKEND_URL;

export async function GET(
    req: NextRequest,
    params: { id: number }
) {
    const token = (await cookies()).get("token")?.value;
    const { id } = await params;
    
    const response = await fetch(`${backend_url}/resume/${id}/preview}`, {
        method: "GET",
        headers: {
            "Authorization": `Bearer ${token}`
        }
    })
    const blob = await response.blob();
    
    return new NextResponse(blob, {
        status: response.status,
        headers: {
            "Content-Type": response.headers.get("Content-Type") || "application/octet-stream"
        }
    })
}