import React, { useCallback, useState } from 'react';
import { UploadCloud, FileType } from 'lucide-react';

export const DropZone = ({ onFileSelect }) => {
  const [isDragOver, setIsDragOver] = useState(false);

  const handleDragOver = useCallback((e) => {
    e.preventDefault();
    setIsDragOver(true);
  }, []);

  const handleDragLeave = useCallback((e) => {
    e.preventDefault();
    setIsDragOver(false);
  }, []);

  const handleDrop = useCallback((e) => {
    e.preventDefault();
    setIsDragOver(false);

    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      const file = e.dataTransfer.files[0];
      if (file.name.endsWith('.xmind')) {
        onFileSelect(file);
      } else {
        alert("Please upload a valid .xmind file");
      }
    }
  }, [onFileSelect]);

  const handleFileInput = (e) => {
    if (e.target.files && e.target.files[0]) {
      onFileSelect(e.target.files[0]);
    }
  };

  return (
    <div
      className={`glass-panel p-12 m-8 flex flex-col items-center justify-center border-2 border-dashed transition-all duration-300 min-h-[400px]
        ${isDragOver ? 'border-blue-500 bg-blue-500/10' : 'border-gray-600 hover:border-gray-500'}`}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
    >
      <div className="bg-gray-800 p-6 rounded-full mb-6 relative group">
        <div className="absolute inset-0 bg-blue-500/20 rounded-full blur-xl group-hover:blur-2xl transition-all"></div>
        <UploadCloud className="w-12 h-12 text-blue-400 relative z-10" />
      </div>

      <h2 className="text-2xl font-bold mb-3">Upload your Xmind file</h2>
      <p className="text-gray-400 mb-8 text-center max-w-md">
        Drag and drop your .xmind file here, or click to browse.
        <br />
        <span className="text-sm opacity-60">Everything is processed locally in your browser.</span>
      </p>

      <label className="btn-primary flex items-center gap-2">
        <FileType className="w-4 h-4" />
        <span>Choose File</span>
        <input
          type="file"
          accept=".xmind"
          className="hidden"
          onChange={handleFileInput}
        />
      </label>
    </div>
  );
};
