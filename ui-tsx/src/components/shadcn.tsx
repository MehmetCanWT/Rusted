import { jsx, RustedVNode } from "../runtime/jsx-runtime";

export type BadgeVariant = "primary" | "secondary" | "outline" | "destructive" | "success";
export type LucideIconName =
  | "sparkles"
  | "cpu"
  | "palette"
  | "code"
  | "zap"
  | "layers"
  | "sliders"
  | "package"
  | "heart"
  | "star"
  | "shield"
  | "sword"
  | "flame"
  | "terminal"
  | "check"
  | "settings";

export function Card(props: { id?: string; className?: string; children?: unknown }): RustedVNode {
  return jsx("card", {
    id: props.id,
    class: `shadcn-card ${props.className ?? ""}`.trim(),
    children: props.children,
  });
}

export function Badge(props: {
  id?: string;
  variant?: BadgeVariant;
  className?: string;
  children?: unknown;
}): RustedVNode {
  const variantCls = props.variant ? `badge-${props.variant}` : "badge-primary";
  return jsx("badge", {
    id: props.id,
    variant: props.variant ?? "primary",
    class: `sdv-badge ${variantCls} ${props.className ?? ""}`.trim(),
    children: props.children,
  });
}

export function Switch(props: {
  id: string;
  checked?: boolean;
  onClick: string;
  tooltip?: string;
}): RustedVNode {
  return jsx("switch", {
    id: props.id,
    checked: props.checked ? "true" : "false",
    "on:click": props.onClick,
    tooltip: props.tooltip,
  });
}

export function LucideIcon(props: {
  name: LucideIconName;
  size?: "sm" | "md" | "lg";
  className?: string;
}): RustedVNode {
  const sizeCls = `lucide-${props.size ?? "md"}`;
  return jsx("icon", {
    name: props.name,
    class: `lucide-icon ${sizeCls} ${props.className ?? ""}`.trim(),
  });
}

export function ThemeSwitcherBar(): RustedVNode {
  return jsx("div", {
    class: "flex flex-row items-center justify-between w-full px-3 py-1 sdv-panel",
    children: [
      jsx("div", {
        class: "flex flex-row items-center gap-2",
        children: [
          LucideIcon({ name: "palette", size: "md" }),
          jsx("span", {
            id: "lbl_active_theme",
            class: "font-small text-sdv-dark",
            children: "Theme: Pelican Classic",
          }),
        ],
      }),
      jsx("div", {
        class: "flex flex-row items-center gap-2",
        children: [
          jsx("button", {
            id: "btn_theme_0",
            class: "sdv-btn",
            "on:click": "theme_pelican",
            tooltip: "Vanilla Stardew Wooden & Parchment 9-Slice Theme",
            children: "Pelican",
          }),
          jsx("button", {
            id: "btn_theme_1",
            class: "sdv-btn",
            "on:click": "theme_zinc",
            tooltip: "Shadcn Zinc Dark Theme (#09090B / #18181B / #27272A)",
            children: "Zinc Dark",
          }),
          jsx("button", {
            id: "btn_theme_2",
            class: "sdv-btn",
            "on:click": "theme_arcane",
            tooltip: "Wizard Arcane Violet Theme (#130D26 / #A855F7)",
            children: "Arcane",
          }),
          jsx("button", {
            id: "btn_theme_3",
            class: "sdv-btn",
            "on:click": "theme_joja",
            tooltip: "JojaCorp Industrial Cyber-Blue Theme (#0A1428 / #0078FF)",
            children: "Joja",
          }),
          jsx("button", {
            id: "btn_theme_4",
            class: "sdv-btn",
            "on:click": "theme_junimo",
            tooltip: "Junimo Forest Emerald Theme (#0F2215 / #22C55E)",
            children: "Junimo",
          }),
        ],
      }),
    ],
  });
}
