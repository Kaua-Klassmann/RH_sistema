import { cookies } from "next/headers";
import { TableBody, TableCell, TableRow } from "./ui/table"
import TableCheckBox from "./table-checkbox";

const backend_url = process.env.backend_url;

type Resume = {
  id: number;
  hired: boolean;
  name: string;
  cpf: string;
  sector: string;
  interview_date: string;
};

async function getResumes() {
    const token = (await cookies()).get("token")?.value;

    const response = await fetch(`${backend_url}/resume/list/1`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${token}`
        },
        body: JSON.stringify({
            "search": "",
            "id_sector": 0,
            "uninterviewed": false
        })
    });

    return (await response.json()).resumes
}

export default async function TableBodyResume({ ...props }) {
    const resumes: Resume[] = await getResumes();

    return (
        <TableBody {...props}>
            { resumes.map(resume => (
                <TableRow key={resume.id} className="h-1/10">
                    <TableCell className="py-0 text-center">
                        <TableCheckBox id={resume.id} hired={resume.hired}/>
                    </TableCell>
                    <TableCell className="py-0">{ resume.name }</TableCell>
                    <TableCell className="py-0">
                        { resume.cpf.slice(0, 3) + "."
                        + resume.cpf.slice(3, 6) + "." + resume.cpf.slice(6, 9)
                        + "-" + resume.cpf.slice(9)}
                    </TableCell>
                    <TableCell className="py-0">{ resume.sector }</TableCell>
                    <TableCell className="py-0">
                        { resume.interview_date.slice(8, 10) + "/"
                        + resume.interview_date.slice(5, 7)  + "/"
                        + resume.interview_date.slice(0, 4) + " - "
                        + resume.interview_date.slice(11, 16)}
                        </TableCell>
                    <TableCell className="py-0">...</TableCell>
                </TableRow>
            ))}
            { resumes.length < 10 && Array.from({ length: 10 - resumes.length }, (_, i) => i).map(i => (
                <TableRow key={i} className="h-1/10">
                    <TableCell className="py-0"></TableCell>
                    <TableCell className="py-0"></TableCell>
                    <TableCell className="py-0"></TableCell>
                    <TableCell className="py-0"></TableCell>
                    <TableCell className="py-0"></TableCell>
                    <TableCell className="py-0"></TableCell>
                </TableRow>
            ))}
        </TableBody>
    )
}