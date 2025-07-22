import { NextRequest, NextResponse } from 'next/server';
import { loadWasmSingleton } from '@/lib/wasm';

// Define correct types for route parameters according to Next.js 15
type RouteParams = any;

// Helper function to process WASM error
function processWasmError(error: any, filePath: string): NextResponse {
    // Try to parse structured error from WASM
    try {
        let errorData;

        if (typeof error === 'object' && error !== null) {
            // Parse error object if it's already an object
            errorData = error;
        } else if (typeof error === 'string') {
            // See if it's a JSON string we can parse 
            try {
                errorData = JSON.parse(error);
            } catch {
                // Not valid JSON, use as is
                errorData = { type: 'Unknown', message: error };
            }
        } else {
            // Fallback for unknown error format
            errorData = {
                type: 'Unknown',
                message: error?.message || String(error)
            };
        }

        // Handle specific error types
        switch (errorData.type) {
            case 'NotFound':
                return NextResponse.json(
                    { error: `File not found: ${filePath}`, details: errorData.message },
                    { status: 404 }
                );
            case 'IsDirectory':
                return NextResponse.json(
                    { error: `Is a directory: ${filePath}`, details: errorData.message },
                    { status: 400 }
                );
            case 'NotDirectory':
                return NextResponse.json(
                    { error: `Not a directory: ${filePath}`, details: errorData.message },
                    { status: 400 }
                );
            case 'ConfigError':
                return NextResponse.json(
                    { error: `Configuration error: ${filePath}`, details: errorData.message },
                    { status: 500 }
                );
            case 'StorageError':
                return NextResponse.json(
                    { error: `Storage error: ${filePath}`, details: errorData.message },
                    { status: 500 }
                );
            case 'NetworkError':
                return NextResponse.json(
                    { error: `Network error: ${filePath}`, details: errorData.message },
                    { status: 503 }
                );
            case 'IoError':
                return NextResponse.json(
                    { error: `I/O error: ${filePath}`, details: errorData.message },
                    { status: 500 }
                );
            default:
                return NextResponse.json(
                    { error: `Operation failed on ${filePath}`, details: errorData.message },
                    { status: 500 }
                );
        }
    } catch (e) {
        // Fallback for any error parsing issues
        console.error('Error processing WASM error:', e);

        // Legacy string parsing for backward compatibility
        const errorMessage = String(error);

        // Try to determine error type from string
        if (errorMessage.includes('NotFound') || errorMessage.includes('not found')) {
            return NextResponse.json(
                { error: `File not found: ${filePath}`, details: errorMessage },
                { status: 404 }
            );
        } else {
            return NextResponse.json(
                { error: `Internal server error processing ${filePath}`, details: errorMessage },
                { status: 500 }
            );
        }
    }
}

// Handle file operations via admin API
export async function GET(
    request: NextRequest,
    { params }: RouteParams
) {
    let wasmModule;
    try {
        wasmModule = await loadWasmSingleton('Admin');
    } catch (error) {
        console.error("Failed to load WASM module:", error);
        return NextResponse.json(
            {
                error: 'Failed to load WASM module',
                details: error instanceof Error ? error.message : String(error)
            },
            { status: 500 }
        );
    }

    const path = (await params).path;

    // Extract path from dynamic route parameter
    const filePath = path.join('/');

    if (!filePath) {
        return NextResponse.json({ error: 'File path is required' }, { status: 400 });
    }

    console.log(`Admin API request: GET /api/admin/${filePath}`);

    try {
        // Check query parameters
        const searchParams = request.nextUrl.searchParams;
        const checkExists = searchParams.get('exists') === 'true';
        const getAttributes = searchParams.get('attributes') === 'true';

        if (checkExists) {
            const exists = await wasmModule.admin_file_exists(filePath);
            console.log(`Exists check for ${filePath}: ${exists}`);
            return NextResponse.json({ path: filePath, exists });
        } else if (getAttributes) {
            const attributes = await wasmModule.admin_get_file_attributes(filePath);
            console.log(`Got attributes for ${filePath}:`, attributes);
            return NextResponse.json({ path: filePath, attributes });
        } else {
            const textContent = await wasmModule.admin_read_file(filePath);
            console.log(`Read file ${filePath}, got text: ${textContent.substring(0, 50)}...`);
            return NextResponse.json({ path: filePath, content: textContent });
        }
    } catch (error: any) {
        console.error(`Error processing admin GET request for ${filePath}:`, error);
        return processWasmError(error, filePath);
    }
}

export async function POST(
    request: NextRequest,
    { params }: RouteParams
) {
    let wasmModule;
    try {
        wasmModule = await loadWasmSingleton('Admin');
    } catch (error) {
        console.error("Failed to load WASM module:", error);
        return NextResponse.json(
            {
                error: 'Failed to load WASM module',
                details: error instanceof Error ? error.message : String(error)
            },
            { status: 500 }
        );
    }

    const path = (await params).path;

    // Extract path from dynamic route parameter
    const filePath = path.join('/');

    if (!filePath) {
        return NextResponse.json({ error: 'File path is required' }, { status: 400 });
    }

    console.log(`Admin API request: POST /api/admin/${filePath}`);

    try {
        // Parse request body
        const body = await request.json();
        const { content: textContent, is_directory } = body;

        if (is_directory) {
            // Creating a directory
            console.log(`Creating directory ${filePath}`);
            await wasmModule.admin_create_directory(filePath);
            return NextResponse.json({
                success: true,
                path: filePath,
                action: 'directory_created'
            });
        } else if (typeof textContent !== 'string') {
            return NextResponse.json({
                error: 'Request body must contain a \'content\' field as string'
            }, { status: 400 });
        } else {
            // Write file content
            console.log(`Write file ${filePath}, content: ${textContent.substring(0, 50)}...`);
            await wasmModule.admin_write_file(filePath, textContent);
            return NextResponse.json({
                success: true,
                path: filePath,
                action: 'written'
            });
        }
    } catch (error: any) {
        console.error(`Error processing admin POST request for ${filePath}:`, error);
        return processWasmError(error, filePath);
    }
}

// Handle PUT requests the same as POST for consistency
export async function PUT(
    request: NextRequest,
    { params }: RouteParams
) {
    return POST(request, { params });
}

export async function DELETE(
    request: NextRequest,
    { params }: RouteParams
) {
    let wasmModule;
    try {
        wasmModule = await loadWasmSingleton('Admin');
    } catch (error) {
        console.error("Failed to load WASM module:", error);
        return NextResponse.json(
            {
                error: 'Failed to load WASM module',
                details: error instanceof Error ? error.message : String(error)
            },
            { status: 500 }
        );
    }

    const path = (await params).path;

    // Extract path from dynamic route parameter
    const filePath = path.join('/');

    if (!filePath) {
        return NextResponse.json({ error: 'File path is required' }, { status: 400 });
    }

    console.log(`Admin API request: DELETE /api/admin/${filePath}`);

    try {
        console.log(`Delete file ${filePath}`);
        await wasmModule.admin_delete_file(filePath);
        return NextResponse.json({
            success: true,
            path: filePath,
            action: 'deleted'
        });
    } catch (error: any) {
        console.error(`Error processing admin DELETE request for ${filePath}:`, error);
        return processWasmError(error, filePath);
    }
} 