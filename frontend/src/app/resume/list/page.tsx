"use client"

import AppSidebar from "@/components/sidebar";
import TableBodyResume from "@/components/table-body-resume";
import { Table, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { useEffect, useState } from "react";
import Pagination from "@/components/pagination";
import { Input } from "@/components/ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { useRouter } from "next/navigation";
import { toast } from "sonner";

type Sector = {
    id: number,
    name: string
}

export default function ListResumes() {
    const router = useRouter();

    const [page, setPage] = useState(1);
    const [search, setSearch] = useState("");
    const [sector, setSector] = useState("0");
    const [uninterviewed, setUninterviewed] = useState("false");

    const [sectors, setSectors] = useState([] as Sector[]);
    const [disabled, setDisabled] = useState(true);

    useEffect(() => {
        async function getSectors() {
            const response = await fetch("/api/sector/list");

            if(response.status == 401) {
                toast.error("Login expirado");
                router.push("/login")

                return;
            }

            if(!response.ok) {
                toast.error("Erro indefinido")
                return;
            }

            const data = await response.json();

            setSectors(data.setores)
            setDisabled(false)
        }

        getSectors()
    }, [router])

    return (
        <AppSidebar page="Listagem dos currículos">
            <main className="mt-10 grid place-items-center">
                <div className="w-[51dvw]">
                    <div className="mb-4 flex gap-2">
                        <Input
                            type="text"
                            placeholder="Nome ou CPF"
                            value={search}
                            onChange={e => setSearch(e.target.value)}
                            className="flex-1/2"
                            disabled={disabled}
                        />
                        <Select onValueChange={setSector}>
                            <SelectTrigger className="flex-1/4" disabled={disabled}>
                                <SelectValue placeholder="Setor" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="0">Todos os Setores</SelectItem>
                                { sectors.map(sector => (
                                    <SelectItem key={sector.id} value={sector.id.toString()}>{ sector.name }</SelectItem>
                                ))}
                            </SelectContent>
                        </Select>
                        <Select onValueChange={setUninterviewed}>
                            <SelectTrigger className="flex-1/4" disabled={disabled}>
                                <SelectValue placeholder="Entrevista" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value={"false"}>Todos</SelectItem>
                                <SelectItem value={"true"}>Não Entrevistado</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>
                    <Table className="mx-auto border">
                        <TableHeader className="bg-muted">
                            <TableRow>
                                <TableHead>Contratado</TableHead>
                                <TableHead className="min-w-45 max-w-45">Nome</TableHead>
                                <TableHead className="min-w-30 max-w-30">Cpf</TableHead>
                                <TableHead className="min-w-30 max-w-30">Setor</TableHead>
                                <TableHead className="min-w-34 max-w-34">Data da Entrevista</TableHead>
                                <TableHead className="min-w-10 max-w-10"></TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBodyResume
                            page={page}
                            search={search}
                            sector={Number(sector)}
                            uninterviewed={uninterviewed === "true"}
                            className="h-90"
                        />
                    </Table>
                    <Pagination page={page} setPage={setPage}/>
                </div>
            </main>
        </AppSidebar>
    )
}