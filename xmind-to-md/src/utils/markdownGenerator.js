/**
 * Converts a simplified topic tree into a Markdown string.
 * @param {Object} rootNode - The root node extracted from Xmind
 * @param {boolean} embedImages - If true, embed images as Data URLs; if false, use file references
 * @returns {string} The formatted Markdown string
 */
export const convertToMarkdown = (rootNode, embedImages = false) => {
    if (!rootNode) return "";

    const processNode = (node, depth) => {
        let md = "";
        const indent = "  ".repeat(Math.max(0, depth - 2));

        // Title handling
        let titleLine = "";
        const title = node.isSummary ? `[Summary] ${node.title}` : node.title;
        if (depth === 0) {
            titleLine = `# ${title}\n`;
        } else if (depth === 1) {
            titleLine = `## ${title}\n`;
        } else if (depth === 2) {
             titleLine = `### ${title}\n`;
        } else {
            titleLine = `${indent}- ${title}\n`;
        }
        md += titleLine;

        // Image handling (if available)
        if (node.imageUrl) {
            const prefix = depth > 2 ? indent + "  " : "";
            if (embedImages) {
                // Embed the Data URL directly
                md += `${prefix}![${node.title}](${node.imageUrl})\n`;
            } else {
                // Use file reference
                const imageName = `${node.id}.png`;
                md += `${prefix}![${node.title}](images/${imageName})\n`;
            }
        }

        // Note handling
        if (node.note) {
             const prefix = depth > 2 ? indent + "  " : "";
             // Split note by lines and indent/quote
             const noteLines = node.note.split('\n').map(line => `${prefix}> ${line}`).join('\n');
             md += `${noteLines}\n\n`;
        } else {
            md += "\n";
        }

        // Process children
        if (node.children && node.children.length > 0) {
            node.children.forEach(child => {
                md += processNode(child, depth + 1);
            });
        }

        return md;
    };

    return processNode(rootNode, 0);
};

/**
 * Extracts all images from the topic tree
 * @param {Object} rootNode - The root node extracted from Xmind
 * @returns {Array} Array of {id, title, dataUrl} objects
 */
export const extractImages = (rootNode) => {
    if (!rootNode) return [];

    const images = [];

    const traverse = (node) => {
        if (node.imageUrl) {
            images.push({
                id: node.id,
                title: node.title,
                dataUrl: node.imageUrl
            });
        }

        if (node.children && node.children.length > 0) {
            node.children.forEach(child => traverse(child));
        }
    };

    traverse(rootNode);
    return images;
};
