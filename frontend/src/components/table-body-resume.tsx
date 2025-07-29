"use client"

import { TableBody, TableCell, TableRow } from "./ui/table"
import TableCheckBox from "./table-checkbox";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import TableDropdownMenu from "./table-dropdown-menu";

interface TableBodyResumeProps extends React.ComponentProps<"tbody"> {
    page: number
}

type Resume = {
  id: number;
  hired: boolean;
  name: string;
  cpf: string;
  sector: string;
  interview_date: string;
};

export default function TableBodyResume({ page, ...props }: TableBodyResumeProps) {
    const [resumes, setResumes] = useState([] as Resume[]);

    const router = useRouter();

    useEffect(() => {
        fetch(`/api/resume/list?page=${page}`, {
            method: "GET"
        })
        .then(async (res) => {
            if (res.status === 401) router.push("/login");
            if (res.ok) {
                setResumes((await res.json()).resumes);
            }
        })
        .catch(() => setResumes([]));
    }, [page, router]);

    return (
        <TableBody {...props}>
            { resumes.map(resume => (
                <TableRow key={resume.id} className="h-1/10">
                    <TableCell className="py-0 text-center">
                        <TableCheckBox id={resume.id} hired={resume.hired}/>
                    </TableCell>
                    <TableCell className="py-0 min-w-45 max-w-45 truncate">{ resume.name }</TableCell>
                    <TableCell className="py-0 min-w-30 max-w-30">
                        { resume.cpf.slice(0, 3) + "."
                        + resume.cpf.slice(3, 6) + "." + resume.cpf.slice(6, 9)
                        + "-" + resume.cpf.slice(9)}
                    </TableCell>
                    <TableCell className="py-0 min-w-30 max-w-30 truncate">{ resume.sector }</TableCell>
                    <TableCell className="py-0 min-w-34 max-w-34">
                        { resume.interview_date.slice(8, 10) + "/"
                        + resume.interview_date.slice(5, 7)  + "/"
                        + resume.interview_date.slice(0, 4) + " - "
                        + resume.interview_date.slice(11, 16)}
                        </TableCell>
                    <TableCell className="min-w-10 max-w-10 p-0">
                        <TableDropdownMenu resumeId={resume.id} className="float-right px-2"/>
                    </TableCell>
                </TableRow>
            ))}
            { resumes.length < 10 && Array.from({ length: 10 - resumes.length }, (_, i) => i).map(i => (
                <TableRow key={i} className="h-1/10">
                    <TableCell className="py-0" />
                    <TableCell className="py-0" />
                    <TableCell className="py-0" />
                    <TableCell className="py-0" />
                    <TableCell className="py-0" />
                    <TableCell className="py-0" />
                </TableRow>
            ))}
        </TableBody>
    )
}