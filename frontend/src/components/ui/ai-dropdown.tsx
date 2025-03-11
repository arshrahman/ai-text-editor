"use client";

import { useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Command } from "@/services/api";

interface AiDropdownProps {
  onSelect: (operation: Command) => void;
}

export function AiDropdown({ onSelect }: AiDropdownProps) {
  const [isOpen, setIsOpen] = useState(false);
  const handleOperation = (operation: Command) => {
    onSelect(operation);
    setIsOpen(false);
  };

  return (
    <DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
      <DropdownMenuTrigger 
        className="flex items-center gap-1 px-2 py-1 text-sm rounded hover:bg-gray-100 group"
        onMouseEnter={() => setIsOpen(true)}
        onMouseLeave={() => setIsOpen(false)}
      >
        🤖 Ask AI
      </DropdownMenuTrigger>
      <DropdownMenuContent 
        onMouseEnter={() => setIsOpen(true)}
        onMouseLeave={() => setIsOpen(false)}
      >
        <DropdownMenuItem onClick={() => handleOperation("summarize")}>
          Summarize
        </DropdownMenuItem>
        <DropdownMenuItem onClick={() => handleOperation("paraphrase")}>
          Paraphrase
        </DropdownMenuItem>
        <DropdownMenuItem onClick={() => handleOperation("expand")}>
          Expand
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}