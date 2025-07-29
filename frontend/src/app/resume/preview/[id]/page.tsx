import { cookies } from "next/headers";
import Image from "next/image";

const backend_url = process.env.BACKEND_URL;

export default async function Preview({
    params
}: { params: Promise<{ id: number}> }) {
    const token = (await cookies()).get("token")?.value;
    const { id } = await params;
    
    const response = await fetch(`${backend_url}/resume/${id}/preview`, {
        method: "GET",
        headers: {
            "Authorization": `Bearer ${token}`
        }
    })

    const buffer = await response.arrayBuffer();
    const base64 = Buffer.from(buffer).toString('base64');

    const contentType = response.headers.get("content-type") || "image/jpeg";

    const dataUrl = `data:${contentType};base64,${base64}`;

    return (
        <div className="min-h-100">
            { contentType.startsWith("image") && 
                <Image src={dataUrl} alt="preview" height={100} width={100} className="min-w-[100dvw]"></Image>
            }
            { !contentType.startsWith("image") &&
                <iframe src={dataUrl} className="min-w-[100dvw] min-h-[100dvh]"></iframe>
            }
        </div>
    );
} 