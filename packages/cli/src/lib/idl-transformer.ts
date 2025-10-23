export function hasLegacyEvents(idlJson: any): boolean {
    if (!idlJson.events || !Array.isArray(idlJson.events) || idlJson.events.length === 0) {
        return false;
    }
    
    return idlJson.events.some((event: any) => event.fields && Array.isArray(event.fields));
}

function generateDiscriminator(eventName: string, index: number): number[] {
    const cpiEventsDiscriminator = [228, 69, 165, 46, 81, 203, 154, 29];
    
    const nameBytes = Buffer.from(eventName, 'utf8');
    const eventSpecific: number[] = [];
    
    for (let i = 0; i < 8; i++) {
        if (i < nameBytes.length) {
            eventSpecific.push(nameBytes[i]);
        } else {
            eventSpecific.push((nameBytes.length + index + i) % 256);
        }
    }
    
    return [...cpiEventsDiscriminator, ...eventSpecific];
}

function convertEventToType(event: any): any {
    return {
        name: event.name,
        type: {
            kind: "struct",
            fields: event.fields.map((field: any) => ({
                name: field.name,
                type: field.type
            }))
        }
    };
}

function convertEventToModern(event: any, index: number): any {
    return {
        name: event.name,
        discriminator: generateDiscriminator(event.name, index)
    };
}

export function transformLegacyEvents(idlJson: any): any {
    if (!hasLegacyEvents(idlJson)) {
        return idlJson;
    }
    
    const transformedIdl = JSON.parse(JSON.stringify(idlJson));
    
    const convertedEventTypes = transformedIdl.events.map((event: any) => convertEventToType(event));
    
    if (!transformedIdl.types) {
        transformedIdl.types = [];
    }
    transformedIdl.types = [...transformedIdl.types, ...convertedEventTypes];
    
    const modernEvents = transformedIdl.events.map((event: any, index: number) => 
        convertEventToModern(event, index)
    );
    
    transformedIdl.events = modernEvents;
    
    return transformedIdl;
}

export function getTransformationInfo(idlJson: any): { hasLegacy: boolean; eventCount: number; eventNames: string[] } {
    const hasLegacy = hasLegacyEvents(idlJson);
    const eventCount = hasLegacy ? idlJson.events.length : 0;
    const eventNames = hasLegacy ? idlJson.events.map((e: any) => e.name) : [];
    
    return { hasLegacy, eventCount, eventNames };
}
