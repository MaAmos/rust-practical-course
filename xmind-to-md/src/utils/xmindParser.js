import JSZip from 'jszip';

/**
 * Parses an Xmind file and extracts the content structure.
 * @param {File} file - The .xmind file object
 * @returns {Promise<Object>} The parsed content object with augmented node data (images/notes)
 */
export const parseXmindFile = async (file) => {
  try {
    const zip = await JSZip.loadAsync(file);

    // Xmind files usually have a content.json file
    if (!zip.file("content.json")) {
      throw new Error("Invalid Xmind file: content.json not found");
    }

    const contentJson = await zip.file("content.json").async("string");
    const content = JSON.parse(contentJson);

    if (!Array.isArray(content) || content.length === 0) {
      throw new Error("Invalid Xmind content structure");
    }

    // Process the first sheet and extract nodes with images/notes
    return await extractNodes(content, zip);
  } catch (error) {
    console.error("Error parsing Xmind file:", error);
    throw error;
  }
};

/**
 * Extracts a simplified tree structure from the Xmind content.
 * @param {Array} content - The parsed content JSON content
 * @param {JSZip} zip - The JSZip instance to extract images
 * @returns {Promise<Object>} Simplified tree with id, title, children, images, and notes
 */
export const extractNodes = async (content, zip) => {
    const firstSheet = content[0];
    if (!firstSheet || !firstSheet.rootTopic) {
        return null;
    }

    const traverse = async (topic) => {
        const node = {
            id: topic.id,
            title: topic.title,
            children: [],
            note: null,
            imageUrl: null
        };

        // Extract Notes
        if (topic.notes && topic.notes.plain && topic.notes.plain.content) {
            node.note = topic.notes.plain.content.trim();
        }

        // Extract Image
        if (topic.image && topic.image.src) {
            const src = topic.image.src;
            // Xmind images are usually stored as 'xap:resources/...'
            // In the zip, they are at 'resources/...'
            if (src.startsWith('xap:')) {
                const path = src.substring(4); // Remove 'xap:'
                // Ensure no leading slash for zip lookup
                const zipPath = path.startsWith('/') ? path.substring(1) : path;

                const imageFile = zip.file(zipPath);
                if (imageFile) {
                    try {
                        const blob = await imageFile.async('blob');
                        // Convert Blob to Data URL (Base64) for better compatibility with html-to-image
                        node.imageUrl = await new Promise((resolve) => {
                            const reader = new FileReader();
                            reader.onloadend = () => resolve(reader.result);
                            reader.readAsDataURL(blob);
                        });
                    } catch (e) {
                        console.warn(`Failed to extract image at ${zipPath}`, e);
                    }
                }
            }
        }

        // Process Children (attached)
        if (topic.children && topic.children.attached) {
            // Use Promise.all to handle async image extraction in children
            node.children = await Promise.all(topic.children.attached.map(traverse));
        }

        return node;
    };

    return await traverse(firstSheet.rootTopic);
};
