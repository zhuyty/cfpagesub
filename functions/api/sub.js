// Cloudflare Pages Function for subscription conversion
export async function onRequest(context) {
  const { request, env } = context;
  
  // Handle CORS
  if (request.method === 'OPTIONS') {
    return new Response(null, {
      status: 200,
      headers: {
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
        'Access-Control-Allow-Headers': 'Content-Type',
      },
    });
  }

  try {
    // For now, return a simple response
    // TODO: Implement actual subscription conversion logic
    const url = new URL(request.url);
    const target = url.searchParams.get('target') || 'clash';
    const subUrl = url.searchParams.get('url');
    
    if (!subUrl) {
      return new Response(JSON.stringify({ error: 'Missing subscription URL' }), {
        status: 400,
        headers: {
          'Content-Type': 'application/json',
          'Access-Control-Allow-Origin': '*',
        },
      });
    }

    // Simple response for testing
    const response = {
      message: 'Subscription conversion API is working',
      target,
      url: subUrl,
      timestamp: new Date().toISOString(),
    };

    return new Response(JSON.stringify(response), {
      status: 200,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
      },
    });
  } catch (error) {
    return new Response(JSON.stringify({ error: error.message }), {
      status: 500,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
      },
    });
  }
}
