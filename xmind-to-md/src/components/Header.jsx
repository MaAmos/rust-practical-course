import React from 'react';
import { FileText, Sparkles } from 'lucide-react';

export const Header = () => {
  return (
    <header className="flex items-center justify-between px-8 py-6 glass-panel mx-6 mt-6">
      <div className="flex items-center gap-3">
        <div className="p-2 bg-blue-500/20 rounded-lg">
          <FileText className="w-6 h-6 text-blue-400" />
        </div>
        <div>
          <h1 className="text-xl font-bold tracking-wide">
            Xmind <span className="text-gray-500 mx-1">to</span> <span className="gradient-text">Markdown</span>
          </h1>
        </div>
      </div>

      <div className="flex items-center gap-2 text-sm text-gray-400">
        <Sparkles className="w-4 h-4 text-yellow-500" />
        <span>Premium Converter</span>
      </div>
    </header>
  );
};
