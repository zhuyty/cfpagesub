import { NextRequest, NextResponse } from 'next/server';
import { loadWasmSingleton } from '@/lib/wasm';

const DOWNLOADS_CACHE_FILE = 'downloads/available_downloads.json';

interface PlatformConfig {
    repo: string;
    asset_pattern: string;
    fallback_url: string;
}

interface AppDownload {
    name: string;
    description: string;
    platforms: Record<string, PlatformConfig>;
}

/**
 * Read the downloads cache from the virtual file system
 */
async function readDownloadsCache(): Promise<{ downloads: AppDownload[], timestamp: number } | null> {
    try {
        const wasmModule = await loadWasmSingleton('Admin');
        const exists = await wasmModule.admin_file_exists(DOWNLOADS_CACHE_FILE);

        if (!exists) {
            return null;
        }

        const cacheData = await wasmModule.admin_read_file(DOWNLOADS_CACHE_FILE);
        return JSON.parse(cacheData);
    } catch (error) {
        console.error('Error reading downloads cache:', error);
        return null;
    }
}

/**
 * Write the downloads cache to the virtual file system
 */
async function writeDownloadsCache(downloads: AppDownload[]): Promise<boolean> {
    try {
        const wasmModule = await loadWasmSingleton('Admin');

        // Ensure the downloads directory exists
        try {
            const dirExists = await wasmModule.admin_file_exists('downloads');
            if (!dirExists) {
                await wasmModule.admin_create_directory('downloads');
            }
        } catch (err) {
            console.error('Error creating downloads directory:', err);
        }

        // Store downloads with current timestamp
        const cacheData = {
            timestamp: Math.floor(Date.now() / 1000),
            downloads: downloads
        };

        await wasmModule.admin_write_file(
            DOWNLOADS_CACHE_FILE,
            JSON.stringify(cacheData, null, 2)
        );

        return true;
    } catch (error) {
        console.error('Error writing downloads cache:', error);
        return false;
    }
}

// GET handler to retrieve the current downloads cache
export async function GET(request: NextRequest) {
    try {
        const cache = await readDownloadsCache();

        if (!cache) {
            return NextResponse.json(
                { error: 'Downloads cache not found' },
                { status: 404 }
            );
        }

        return NextResponse.json(cache);
    } catch (error) {
        console.error('Error retrieving downloads cache:', error);
        return NextResponse.json(
            { error: 'Failed to retrieve downloads cache' },
            { status: 500 }
        );
    }
}

// POST handler to update the downloads cache
export async function POST(request: NextRequest) {
    try {
        const body = await request.json();

        // Validate input
        if (!body.downloads || !Array.isArray(body.downloads)) {
            return NextResponse.json(
                { error: 'Invalid request body. Expected an array of downloads.' },
                { status: 400 }
            );
        }

        const success = await writeDownloadsCache(body.downloads);

        if (!success) {
            return NextResponse.json(
                { error: 'Failed to update downloads cache' },
                { status: 500 }
            );
        }

        return NextResponse.json({
            success: true,
            message: 'Downloads cache updated successfully'
        });
    } catch (error) {
        console.error('Error updating downloads cache:', error);
        return NextResponse.json(
            { error: 'Failed to update downloads cache' },
            { status: 500 }
        );
    }
}

// PUT handler for consistency with other admin endpoints
export async function PUT(request: NextRequest) {
    return POST(request);
} 