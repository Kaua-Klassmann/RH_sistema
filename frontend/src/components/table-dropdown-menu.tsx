import { Button } from "./ui/button";
import { TableOfContents } from "lucide-react";
import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuTrigger } from "./ui/dropdown-menu";

interface Props extends React.ComponentProps<"div">{
    resumeId: number
}

export default function TableDropdownMenu({ id, ...props }: Props) {
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
                            window.open(`/resume/preview/${id}`, "_blank")
                        }}>Visualizar currículo</DropdownMenuItem>
                    </DropdownMenuGroup>
                </DropdownMenuContent>
            </DropdownMenu>
        </div>
    )
}