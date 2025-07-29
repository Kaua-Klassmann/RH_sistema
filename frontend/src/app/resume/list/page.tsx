"use client"

import AppSidebar from "@/components/sidebar";
import TableBodyResume from "@/components/table-body-resume";
import { Table, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { useState } from "react";
import Pagination from "@/components/pagination";

export default function ListResumes() {
    const [page, setPage] = useState(1);

    return (
        <AppSidebar page="Listagem dos currículos">
            <main className="mt-8 grid place-items-center">
                <div className="w-[51dvw]">
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
                        <TableBodyResume page={page} className="h-90"/>
                    </Table>
                    <Pagination page={page} setPage={setPage}/>
                </div>
            </main>
        </AppSidebar>
    )
}