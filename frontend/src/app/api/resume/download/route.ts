import { cookies } from "next/headers";
import { NextRequest, NextResponse } from "next/server";

const backend_url = process.env.BACKEND_URL;

export async function GET(req: NextRequest,) {
  const token = (await cookies()).get("token")?.value;

  if(!token) {
    return NextResponse.json({}, { status: 401 });
  }

  const searchParams = req.nextUrl.searchParams;
  const id = searchParams.get("id");

  if(!id) {
    return NextResponse.json({}, { status: 400 })
  }

  const response = await fetch(`${backend_url}/resume/${id}/download`, {
    method: "GET",
    headers: {
      "Authorization": `Bearer ${token}`
    }
  })

  if(!response.ok) {
    return NextResponse.json({}, { status: 400 })
  }

  const buffer = await response.arrayBuffer();

  const contentType = response.headers.get("content-type") || "application/octet-stream";
  const contentDisposition = response.headers.get("content-disposition") || "attachment";

  return new NextResponse(buffer, {
    status: 200,
    headers: {
      "Content-Type": contentType,
      "Content-Disposition": contentDisposition,
    },
  });
}
