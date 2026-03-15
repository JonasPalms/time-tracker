import changelog from "../../../CHANGELOG.md?raw";

export type ChangelogSection = {
  title: string;
  items: string[];
};

function normalizeVersion(version: string) {
  return version.trim().replace(/^v/i, "");
}

export function getChangelogSections(version: string): ChangelogSection[] {
  const normalizedVersion = normalizeVersion(version);
  const escapedVersion = normalizedVersion.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const versionRegex = new RegExp(`^## \\[(?:v)?${escapedVersion}\\].*$`, "m");
  const match = changelog.match(versionRegex);

  if (!match || match.index === undefined) {
    return [];
  }

  const sectionStart = match.index + match[0].length;
  const remaining = changelog.slice(sectionStart);
  const nextVersionMatch = remaining.match(/^## \[/m);
  const sectionBody = (
    nextVersionMatch ? remaining.slice(0, nextVersionMatch.index) : remaining
  ).trim();

  if (!sectionBody) {
    return [];
  }

  const sections: ChangelogSection[] = [];
  let currentSection: ChangelogSection | null = null;

  for (const line of sectionBody.split("\n")) {
    const trimmed = line.trim();

    if (!trimmed) {
      continue;
    }

    if (trimmed.startsWith("### ")) {
      currentSection = {
        title: trimmed.slice(4).trim(),
        items: [],
      };
      sections.push(currentSection);
      continue;
    }

    if (trimmed.startsWith("- ")) {
      if (!currentSection) {
        currentSection = {
          title: "Highlights",
          items: [],
        };
        sections.push(currentSection);
      }

      currentSection.items.push(trimmed.slice(2).trim());
    }
  }

  return sections.filter((section) => section.items.length > 0);
}
