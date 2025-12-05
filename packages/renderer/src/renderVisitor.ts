import { deleteDirectory, writeRenderMapVisitor } from '@codama/renderers-core';
import { rootNodeVisitor, visit } from '@codama/visitors-core';
import { getRenderMapVisitor, GetRenderMapOptions } from './getRenderMapVisitor';

export type RenderOptions = GetRenderMapOptions & {
    deleteFolderBeforeRendering?: boolean;
};

export function renderVisitor(path: string, options: RenderOptions = {}) {
    return rootNodeVisitor(root => {
        // Delete existing generated folder
        if (options.deleteFolderBeforeRendering ?? true) {
            deleteDirectory(path);
        }

        // Render the new files
        visit(root, writeRenderMapVisitor(getRenderMapVisitor(options), path));
    });
}
