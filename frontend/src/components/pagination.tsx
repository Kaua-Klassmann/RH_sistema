"use client"

import { IconChevronLeft, IconChevronRight, IconChevronsLeft, IconChevronsRight } from "@tabler/icons-react";
import { Button } from "./ui/button";
import { toast } from "sonner";

type PaginationProps = {
    page: number,
    setPage: (value: number) => void
}

export default function Pagination({ page, setPage }: PaginationProps) {
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
        if(page == 2) {
            toast.warning("Nenhuma página encontrada");

            return;
        }

        setPage(page + 1)
    }

    return (
        <div className="w-full py-2 flex flex-row justify-center gap-2">
            <Button variant="outline" className="hidden h-8 w-8 p-0 lg:flex" onClick={setMinPage}>
                <IconChevronsLeft />
            </Button>
            <Button variant="outline" className="hidden h-8 w-8 p-0 lg:flex" onClick={decreasePage}>
                <IconChevronLeft />
            </Button>
            <Button variant="outline" className="hidden h-8 w-8 p-0 lg:flex" onClick={incrementPage}>
                <IconChevronRight />
            </Button>
            <Button variant="outline" className="hidden h-8 w-8 p-0 lg:flex">
                <IconChevronsRight />
            </Button>
        </div>
    )
}