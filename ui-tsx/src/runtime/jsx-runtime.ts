// Zero-DOM JSX/TSX Runtime for RustedUI (Taffy + SlotMap Native Compiler)

export interface RustedVNode {
  tag: string;
  props: Record<string, unknown>;
  children: Array<RustedVNode | string>;
}

export function escapeXml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

export function jsx(
  type: string | ((props: Record<string, unknown>) => RustedVNode),
  props: Record<string, unknown>
): RustedVNode {
  if (typeof type === "function") {
    return type(props);
  }
  const { children, ...rest } = props;
  const childList = Array.isArray(children)
    ? children.flat().filter((c) => c !== null && c !== undefined && c !== false)
    : children !== null && children !== undefined && children !== false
    ? [children]
    : [];

  return {
    tag: type,
    props: rest,
    children: childList.map((c) => (typeof c === "object" ? (c as RustedVNode) : String(c))),
  };
}

export const jsxs = jsx;

export function Fragment(props: { children?: unknown }): RustedVNode {
  return jsx("div", { class: "flex flex-col", children: props.children });
}

export function renderToRustedMarkup(node: RustedVNode, indent = 0): string {
  const pad = " ".repeat(indent);
  const attrs = Object.entries(node.props)
    .filter(([, v]) => v !== undefined && v !== null && v !== false)
    .map(([k, v]) => {
      const attrName = k === "className" ? "class" : k === "onClick" ? "on:click" : k;
      return `${attrName}="${escapeXml(String(v))}"`;
    })
    .join(" ");

  const openTag = attrs.length > 0 ? `<${node.tag} ${attrs}` : `<${node.tag}`;
  if (node.children.length === 0) {
    return `${pad}${openTag} />`;
  }

  if (node.children.length === 1 && typeof node.children[0] === "string") {
    return `${pad}${openTag}>${escapeXml(node.children[0])}</${node.tag}>`;
  }

  const inner = node.children
    .map((c) =>
      typeof c === "string"
        ? `${" ".repeat(indent + 4)}${escapeXml(c)}`
        : renderToRustedMarkup(c, indent + 4)
    )
    .join("\n");

  return `${pad}${openTag}>\n${inner}\n${pad}</${node.tag}>`;
}
