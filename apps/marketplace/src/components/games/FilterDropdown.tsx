"use client"

import { useEffect, useId, useRef, useState, type ReactNode } from "react"
import { ChevronDown, Check } from "lucide-react"

type Option = { value: string; label: string }

interface Props {
  label: string
  icon?: ReactNode
  value: string
  options: ReadonlyArray<Option>
  onChange: (v: string) => void
  ariaLabel: string
}

export function FilterDropdown({ label, icon, value, options, onChange, ariaLabel }: Props) {
  const [open, setOpen] = useState(false)
  const [active, setActive] = useState(-1)
  const [menuW, setMenuW] = useState<number | undefined>(undefined)

  const rootRef = useRef<HTMLDivElement>(null)
  const btnRef = useRef<HTMLButtonElement>(null)
  const listboxId = useId()

  const selected = options.find(o => o.value === value)
  const display = selected?.label ?? label

  // Close on outside click
  useEffect(() => {
    const onDocClick = (e: MouseEvent) => {
      if (!rootRef.current?.contains(e.target as Node)) {
        setOpen(false)
        setActive(-1)
      }
    }
    document.addEventListener("mousedown", onDocClick)
    return () => document.removeEventListener("mousedown", onDocClick)
  }, [])

  // Keyboard nav while menu is open
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (!open) return
      if (e.key === "Escape") {
        setOpen(false); setActive(-1); btnRef.current?.focus(); return
      }
      if (e.key === "ArrowDown") { e.preventDefault(); setActive(p => (p + 1) % options.length); return }
      if (e.key === "ArrowUp") { e.preventDefault(); setActive(p => (p > 0 ? p - 1 : options.length - 1)); return }
    }
    document.addEventListener("keydown", onKey)
    return () => document.removeEventListener("keydown", onKey)
  }, [open, options.length])

  // Keep menu width equal to trigger width
  useEffect(() => {
    const el = btnRef.current
    if (!el) return
    const ro = new ResizeObserver(() => setMenuW(el.offsetWidth))
    ro.observe(el)
    setMenuW(el.offsetWidth)
    return () => ro.disconnect()
  }, [])

  return (
    <div ref={rootRef} className="relative">
      {/* trigger */}
      <button
        ref={btnRef}
        type="button"
        onClick={() => {
          setOpen(o => !o)
          if (!open) setActive(options.findIndex(o => o.value === value))
        }}
        aria-expanded={open}
        aria-haspopup="listbox"
        aria-controls={open ? listboxId : undefined}
        aria-label={ariaLabel}
        className="h-12 w-full min-w-[160px] rounded border border-[#16213E] bg-[#050508]
                   px-[13px] text-[14px] text-white flex items-center gap-2
                   focus:outline-none focus:ring-2 focus:ring-[#00D4FF]"
      >
        {icon ? <span aria-hidden className="grid place-items-center pr-2">{icon}</span> : null}
        <span className="flex-1 text-left">{display}</span>
        <ChevronDown className={`h-4 w-4 text-white/50 transition-transform ${open ? "rotate-180" : ""}`} />
      </button>

      {/* menu */}
      {open && (
        <div
          id={listboxId}
          aria-label={ariaLabel}
          style={{ width: menuW }}
          className="absolute z-50 mt-2 rounded border border-[#16213E] bg-[#050508] py-1
                     shadow-[0_24px_48px_rgba(0,0,0,0.5)]"
        >
          {options.map((opt, i) => {
            const isSelected = opt.value === value
            const isActive = i === active
            return (
              <button
                key={opt.value}
                id={`${listboxId}-opt-${i}`}
                type="button"
                aria-current={isSelected ? "true" : undefined}
                onMouseEnter={() => setActive(i)}
                onClick={() => { onChange(opt.value); setOpen(false); setActive(-1); btnRef.current?.focus() }}
                onKeyDown={(e) => {
                  if (e.key === "Enter" || e.key === " ") {
                    e.preventDefault()
                    onChange(opt.value)
                    setOpen(false); setActive(-1)
                    btnRef.current?.focus()
                  }
                }}
                className={[
                  "relative w-full text-left cursor-pointer px-9 py-2 text-[14px] flex items-center",
                  isSelected ? "text-white" : "text-[#D1D5DB]",
                  !isSelected && isActive ? "bg-white/5" : ""
                ].join(" ")}
              >
                {isSelected && (
                  <span className="absolute inset-y-1 left-1 right-1 rounded bg-[#8B5CF6]" />
                )}
                {isSelected && <Check className="absolute left-3 h-4 w-4 text-white" />}
                <span className="relative">{opt.label}</span>
              </button>
            )
          })}
        </div>
      )}
    </div>
  )
}
