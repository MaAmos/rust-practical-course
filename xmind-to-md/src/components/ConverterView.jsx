import React, { useState, useRef, useCallback } from 'react';
import { Download, Copy, Check, Image as ImageIcon } from 'lucide-react';
import { toPng } from 'html-to-image';
import JSZip from 'jszip';
import { convertToMarkdown, extractImages } from '../utils/markdownGenerator';
import './TreeView.css';

export const ConverterView = ({ fileName, topicTree, mdContent }) => {
  const [copied, setCopied] = useState(false);
  const [exportingImg, setExportingImg] = useState(false);
  const treeRef = useRef(null);

  const handleCopy = () => {
    // Generate Markdown with embedded Data URLs
    const mdWithImages = convertToMarkdown(topicTree, true);
    navigator.clipboard.writeText(mdWithImages);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleDownloadMd = async () => {
    const zip = new JSZip();
    const baseName = fileName ? fileName.replace('.xmind', '') : 'output';

    // Generate Markdown with file references
    const mdWithFileRefs = convertToMarkdown(topicTree, false);
    zip.file(`${baseName}.md`, mdWithFileRefs);

    // Extract and add images
    const images = extractImages(topicTree);
    if (images.length > 0) {
      const imagesFolder = zip.folder('images');
      for (const img of images) {
        // Convert Data URL to Blob
        const base64Data = img.dataUrl.split(',')[1];
        const byteCharacters = atob(base64Data);
        const byteNumbers = new Array(byteCharacters.length);
        for (let i = 0; i < byteCharacters.length; i++) {
          byteNumbers[i] = byteCharacters.charCodeAt(i);
        }
        const byteArray = new Uint8Array(byteNumbers);
        const blob = new Blob([byteArray], { type: 'image/png' });

        imagesFolder.file(`${img.id}.png`, blob);
      }
    }

    // Generate and download ZIP
    const content = await zip.generateAsync({ type: 'blob' });
    const element = document.createElement('a');
    element.href = URL.createObjectURL(content);
    element.download = `${baseName}.zip`;
    document.body.appendChild(element);
    element.click();
    document.body.removeChild(element);
  };

  const handleDownloadImage = useCallback(async () => {
    if (!treeRef.current) return;

    setExportingImg(true);

    // Store original styles to restore later
    const originalStyles = {
      padding: treeRef.current.style.padding,
      backgroundColor: treeRef.current.style.backgroundColor,
      borderRadius: treeRef.current.style.borderRadius,
      transform: treeRef.current.style.transform,
      transformOrigin: treeRef.current.style.transformOrigin
    };

    try {
      // Apply export styles with scale transformation for higher quality
      const scale = 2; // Scale up 2x for better text rendering
      treeRef.current.style.padding = '40px';
      treeRef.current.style.backgroundColor = '#0f172a';
      treeRef.current.style.borderRadius = '12px';
      treeRef.current.style.transform = `scale(${scale})`;
      treeRef.current.style.transformOrigin = 'top left';

      // Increase image size limits for higher quality export (but keep them controlled)
      const images = treeRef.current.getElementsByTagName('img');
      const imageOriginalStyles = [];
      for (let i = 0; i < images.length; i++) {
        const img = images[i];
        imageOriginalStyles.push({
          maxWidth: img.style.maxWidth,
          maxHeight: img.style.maxHeight
        });
        // Set larger limits instead of removing them entirely
        img.style.maxWidth = '400px';
        img.style.maxHeight = '400px';
      }

      // Wait for styles to apply
      await new Promise(resolve => setTimeout(resolve, 300));

      const dataUrl = await toPng(treeRef.current, {
        cacheBust: true,
        backgroundColor: '#0f172a',
        pixelRatio: 2, // Combined with scale for very high resolution
        skipFonts: false
      });

      // Restore image styles
      for (let i = 0; i < images.length; i++) {
        images[i].style.maxWidth = imageOriginalStyles[i].maxWidth;
        images[i].style.maxHeight = imageOriginalStyles[i].maxHeight;
      }

      // Validate data URL
      if (!dataUrl || dataUrl.length < 100) {
          throw new Error("Generated image data is empty or too small.");
      }

      console.log('Data URL length:', dataUrl.length);

      const link = document.createElement('a');
      link.download = fileName ? fileName.replace('.xmind', '.png') : 'mindmap.png';
      link.href = dataUrl;
      link.click();

    } catch (err) {
      console.error('Failed to export image:', err);
      const msg = err instanceof Error ? err.message : String(err);
      alert(`Export failed: ${msg}. Check console for details.`);
    } finally {
      // Restore original styles
      treeRef.current.style.padding = originalStyles.padding;
      treeRef.current.style.backgroundColor = originalStyles.backgroundColor;
      treeRef.current.style.borderRadius = originalStyles.borderRadius;
      treeRef.current.style.transform = originalStyles.transform;
      treeRef.current.style.transformOrigin = originalStyles.transformOrigin;
      setExportingImg(false);
    }
  }, [fileName]);

  // Improved Render Tree for "True Mind Map" Style
  const renderMindMap = (node, isRoot = true) => {
    if (!node) return null;
    const hasChildren = node.children && node.children.length > 0;

    return (
      <div key={node.id} className="flex flex-row items-center">
        {/* The Node Card */}
        <div className={`
            feature-card
            ${isRoot ? 'root' : ''}
            mx-4 my-2
            relative
            z-10
            flex flex-col gap-2 items-center
        `}>
           {node.imageUrl && (
             <img src={node.imageUrl} alt={node.title} className="max-w-[200px] max-h-[200px] rounded-sm object-contain" />
           )}

           <span>{node.title}</span>

           {node.note && (
             <div className="text-[10px] bg-slate-700/50 p-1 rounded w-full text-left text-gray-300 italic whitespace-pre-wrap max-w-[200px]">
               {node.note}
             </div>
           )}

           {/* Connector to children (Right side) */}
           {hasChildren && (
             <div className="absolute right-[-2rem] top-1/2 w-[2rem] h-[2px] bg-slate-600"></div>
           )}
        </div>

        {/* Children Column */}
        {hasChildren && (
           <div className="flex flex-col relative py-2">
              <div className="flex flex-col border-l-2 border-slate-600 border-opacity-0 relative">
                  {node.children.map((child, index, arr) => (
                      <div key={child.id} className="flex flex-row items-center relative pl-8">
                          {/* Horizontal line to child */}
                          <div className="absolute left-0 top-1/2 w-8 h-[2px] bg-slate-600"></div>

                          {/* Vertical Line Connector Logic */}
                           {arr.length > 1 && (
                             <>
                               {/* Line connecting upwards to previous sibling/parent center */}
                               {index > 0 && (
                                 <div className="absolute left-0 top-0 h-1/2 w-[2px] bg-slate-600"></div>
                               )}
                               {/* Line connecting downwards to next sibling */}
                               {index < arr.length - 1 && (
                                 <div className="absolute left-0 top-1/2 h-1/2 w-[2px] bg-slate-600"></div>
                               )}
                             </>
                           )}

                          {renderMindMap(child, false)}
                      </div>
                  ))}
              </div>
           </div>
        )}
      </div>
    );
  };

  return (
    <div className="flex flex-col h-full overflow-hidden p-6 gap-6">
      {/* Toolbar */}
      <div className="flex justify-between items-center glass-panel px-6 py-4">
        <div>
          <h2 className="font-semibold text-lg">{fileName}</h2>
          <p className="text-xs text-gray-500">Converted successfully</p>
        </div>
        <div className="flex gap-3">
          <button onClick={handleDownloadImage} disabled={exportingImg} className="btn-secondary flex items-center gap-2">
            <ImageIcon className="w-4 h-4 text-purple-400" />
            {exportingImg ? "Generating..." : "Export PNG"}
          </button>
          <div className="h-8 w-[1px] bg-gray-700 mx-1"></div>
          <button onClick={handleCopy} className="btn-secondary flex items-center gap-2">
            {copied ? <Check className="w-4 h-4 text-green-400" /> : <Copy className="w-4 h-4" />}
            {copied ? "Copied" : "Copy MD"}
          </button>
          <button onClick={handleDownloadMd} className="btn-primary flex items-center gap-2">
            <Download className="w-4 h-4" />
            Download .md
          </button>
        </div>
      </div>

      {/* Main Content Split */}
      <div className="flex flex-1 gap-6 overflow-hidden">
        {/* Left: Tree Preview */}
        <div className="w-1/2 glass-panel p-6 overflow-y-auto custom-scrollbar">
          <h3 className="text-sm font-bold text-gray-400 uppercase tracking-widest mb-6 sticky top-0 bg-slate-900/50 backdrop-blur-md py-2 z-10">
            Visual Preview
          </h3>
          <div ref={treeRef} className="font-sans min-w-fit inline-block p-8 bg-slate-900/50 rounded-xl">
            {renderMindMap(topicTree)}
          </div>
        </div>

        {/* Right: Markdown */}
        <div className="flex-1 glass-panel p-6 overflow-hidden flex flex-col">
          <h3 className="text-sm font-bold text-gray-400 uppercase tracking-widest mb-4">Markdown Source</h3>
          <textarea
            className="w-full h-full bg-transparent border-none outline-none resize-none font-mono text-sm text-gray-300 p-2 custom-scrollbar"
            value={mdContent}
            readOnly
          />
        </div>
      </div>
    </div>
  );
};
