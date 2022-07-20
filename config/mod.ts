import { http } from "./deps.ts";

export type BoxerConfiguration = {
    name: string;
}

const handleConfigServerRequest = async (request: Request): Promise<Response> => {
    

    return new Response(null, { status: 505 });
}

const serveConfig = async ({}: BoxerConfiguration) => {
    await http.serve(handleConfigServerRequest, {
        port: 8080
    });
}

export default serveConfig;