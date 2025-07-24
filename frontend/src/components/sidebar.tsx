import { cn } from "@/lib/utils";
import { SidebarInset, SidebarProvider, SidebarTrigger, Sidebar, SidebarContent, SidebarMenuItem, SidebarMenuButton, SidebarMenu, SidebarRail } from "./ui/sidebar";
import Link from "next/link";
import { ModeToggle } from "./mode-toggle";
import LogoffButton from "./logoff-button";

export default function AppSidebar({ className, children, ...props }: React.ComponentProps<"div">) {
    const data = [
        {
            title: "Cadastrar currículo",
            url: "/resume/create"
        },
        {
            title: "Listar currículos",
            url: "/resume/list"
        }
    ];

    return (
        <SidebarProvider className={cn(className)}>
            <Sidebar {...props}>
                <SidebarContent className="py-2 px-1 flex flex-col justify-between">
                    <SidebarMenu>
                        { data.map((item) => (
                            <SidebarMenuItem key={item.title}>
                                <SidebarMenuButton asChild>
                                    <Link href={item.url}>{item.title}</Link>
                                </SidebarMenuButton>
                            </SidebarMenuItem>
                        ))}
                    </SidebarMenu>
                    <SidebarMenu className="flex flex-row justify-center gap-8 px-1">
                        <LogoffButton />
                        <ModeToggle />
                    </SidebarMenu>
                </SidebarContent>
                <SidebarRail />
            </Sidebar>
            <SidebarInset>
                <header className="p-2">
                    <SidebarTrigger className="-ml-1"/>
                </header>
                { children }
            </SidebarInset>
        </SidebarProvider>
    )
}