"use client"

import { ChevronLeft, ChevronsLeft, ChevronsRight, ChevronRight } from "lucide-react";
import { Button } from "./ui/button";
import { toast } from "sonner";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";

type PaginationProps = {
    page: number,
    setPage: (value: number) => void
}

export default function Pagination({ page, setPage }: PaginationProps) {
    const router = useRouter();

    const [max, setMax] = useState(1);
    const [isLoading, setIsLoading] = useState(true);

    function setMinPage() {
        if(page == 1) {
            toast.warning("Nenhuma página encontrada");

            return;
        }

        setPage(1)
    }

    function decreasePage() {
        if(page == 1) {
            toast.warning("Nenhuma página encontrada");

            return;
        }

        setPage(page - 1)
    }

    function incrementPage() {
        if(page == max) {
            toast.warning("Nenhuma página encontrada");

            return;
        }

        setPage(page + 1)
    }

    function setMaxPage() {
        if(page == max) {
            toast.warning("Nenhuma página encontrada");

            return;
        }

        setPage(max)
    }

    useEffect(() => {
        fetch("/api/resume/pages")
            .then(async (res) => {
                if(res.status == 401) {
                    router.push("/login")
                    return null;
                }

                return await res.json()
            })
            .then(res => {
                setMax(res.pages);
                setIsLoading(false);
            })
            .catch(() => toast.error("Falha ao procurar usuários"))
    }, [router])

    return (
        <div className="w-full py-2 flex flex-row justify-center gap-2">
            <Button variant="outline" className="hidden h-8 w-8 p-0 lg:flex" onClick={setMinPage} disabled={isLoading}>
                <ChevronsLeft />
            </Button>
            <Button variant="outline" className="hidden h-8 w-8 p-0 lg:flex" onClick={decreasePage} disabled={isLoading}>
                <ChevronLeft />
            </Button>
            <Button variant="outline" className="hidden h-8 w-8 p-0 lg:flex" onClick={incrementPage} disabled={isLoading}>
                <ChevronRight />
            </Button>
            <Button variant="outline" className="hidden h-8 w-8 p-0 lg:flex" onClick={setMaxPage} disabled={isLoading}>
                <ChevronsRight />
            </Button>
        </div>
    )
}