import { NextRequest, NextResponse } from "next/server";

export function middleware(req: NextRequest) {
    const token = req.cookies.get("token")?.value;

    if (!token &&
        (req.nextUrl.pathname.startsWith("/resume") || req.nextUrl.pathname.startsWith("/api/resume"))
    ) {
        return NextResponse.redirect(new URL("/login", req.url));
    }

    return NextResponse.next();
}