import AppSidebar from "@/components/sidebar";
import TableBodyResume from "@/components/table-body-resume";
import { Table, TableHead, TableHeader, TableRow } from "@/components/ui/table";

export default function ListResumes() {
    return (
        <AppSidebar page="Listagem dos currículos">
            <main className="mt-8">
                <Table className="mx-auto w-auto border">
                    <TableHeader className="bg-muted">
                        <TableRow>
                            <TableHead>Contratado</TableHead>
                            <TableHead>Nome</TableHead>
                            <TableHead>Cpf</TableHead>
                            <TableHead>Setor</TableHead>
                            <TableHead>Data da Entrevista</TableHead>
                            <TableHead></TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBodyResume className="h-80"/>
                </Table>
            </main>
        </AppSidebar>
    )
}