import { Button } from "./ui/button";
import { TableOfContents } from "lucide-react";
import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuTrigger } from "./ui/dropdown-menu";
import { useRouter } from "next/navigation";
import { toast } from "sonner";

interface Props extends React.ComponentProps<"div">{
    resumeId: number
}

export default function TableDropdownMenu({ resumeId, ...props }: Props) {
    const router = useRouter();

    async function downloadResume() {
        const response = await fetch(`/api/resume/download?id=${resumeId}`, {
            method: "GET"
        })

        if(response.status == 401) {
            router.push("/login")
            return;
        }

        if(response.status == 400) {
            toast.error("Erro desconhecido")
            return;
        }

        const blob = await response.blob();
        const url = window.URL.createObjectURL(blob);

        const contentDisposition = response.headers.get("content-disposition");
        const fileNameMatch = contentDisposition?.match(/filename="(.+)"/);
        const fileName = fileNameMatch ? fileNameMatch[1] : "curriculo";

        const a = document.createElement("a");
        a.href = url;
        a.download = fileName;
        document.body.appendChild(a);
        a.click();
        a.remove();
        window.URL.revokeObjectURL(url);
    };

    return (
        <div {...props}>
            <DropdownMenu >
                <DropdownMenuTrigger asChild>
                    <Button variant={"outline"} className="h-7 w-7">
                        <TableOfContents className="h-7 w-7"/>
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="start">
                    <DropdownMenuGroup>
                        <DropdownMenuItem onClick={() => {
                            window.open(`/resume/preview/${resumeId}`, "_blank")
                        }}>Visualizar currículo</DropdownMenuItem>
                        <DropdownMenuItem onClick={() => downloadResume()}>Baixar currículo</DropdownMenuItem>
                    </DropdownMenuGroup>
                </DropdownMenuContent>
            </DropdownMenu>
        </div>
    )
}