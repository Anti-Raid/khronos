/**
 * #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KhronosValue {
    Text(String),
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    Boolean(bool),
    Buffer(Vec<u8>),   // Binary data
    Vector((f32, f32, f32)), // Luau vector
    Map(Vec<(KhronosValue, KhronosValue)>),
    List(Vec<KhronosValue>),
    Timestamptz(chrono::DateTime<chrono::Utc>),
    Interval(chrono::Duration),
    TimeZone(chrono_tz::Tz),
    LazyStringMap(HashMap<String, String>), // For lazy string maps
    Null,
}
 */

type KhronosValue = {
    Text: string;
} | {
    Integer: number;
} | {
    UnsignedInteger: number;
} | {
    Float: number;
} | {
    Boolean: boolean;
} | {
    Buffer: number[];
} | {
    Vector: [number, number, number];
} | {
    Map: [KhronosValue, KhronosValue][];
} | {
    List: KhronosValue[];
} | {
    Timestamptz: string; // ISO 8601 format
} | {
    Interval: number; // Duration in milliseconds
} | {
    TimeZone: string; // Time zone identifier
} | {
    LazyStringMap: Record<string, string>;
} | {
    Null: null;
}

class LazyStringMap {
    public map: Record<string, string>;

    constructor(map: Record<string, string>) {
        this.map = map;
    }
}

class Vector {
    public vector: [number, number, number];
    
    constructor(vector: [number, number, number]) {
        this.vector = vector;
    }
}

class Interval {
    public milliseconds: number;

    constructor(milliseconds: number) {
        this.milliseconds = milliseconds;
    }
}

class TimeZone {
    public timezone: string;

    constructor(timezone: string) {
        this.timezone = timezone;
    }
}

/**
 * Decode a KhronosValue into a nicer to work with JavaScript value
 * @param data The data from the server
 */
const decode = (data: KhronosValue, depth?: number): any => {
    if ((depth || 0) > 100) {
        return null; // Prevent excessive recursion
    }
    if ('Text' in data) {
        return data.Text;
    } else if ('Integer' in data) {
        return data.Integer;
    } else if ('UnsignedInteger' in data) {
        return data.UnsignedInteger;
    } else if ('Float' in data) {
        return data.Float;
    } else if ('Boolean' in data) {
        return data.Boolean;
    } else if ('Buffer' in data) {
        return new Uint8Array(data.Buffer);
    } else if ('Vector' in data) {
        return new Vector(data.Vector);
    } else if ('Map' in data) {
        const obj: Map<any, any> = new Map();
        for (const [key, value] of data.Map) {
            const decodedKey = decode(key, (depth || 0) + 1);
            const decodedValue = decode(value, (depth || 0) + 1);
            obj.set(decodedKey, decodedValue);
        }
        return obj;
    } else if ('List' in data) {
        return data.List.map((item) => decode(item, (depth || 0) + 1));
    } else if ('Timestamptz' in data) {
        return new Date(data.Timestamptz);
    } else if ('Interval' in data) {
        return new Interval(data.Interval);
    } else if ('TimeZone' in data) {
        return new TimeZone(data.TimeZone);
    } else if ('LazyStringMap' in data) {
        return new LazyStringMap(data.LazyStringMap);
    } else if ('Null' in data) {
        return null;
    } else {
        throw new Error('Unknown KhronosValue type');
    }
}

/**
 * Encode a JavaScript value into a KhronosValue, unknown types are encoded using toString()
 * @param value The Value to encode into a KhronosValue
 */
const encode = (value: any): KhronosValue => {
    if (value === null || value === undefined) {
        return { Null: null };
    } else if (typeof value === 'string') {
        return { Text: value };
    } else if (typeof value === 'number') {
        if (Number.isInteger(value)) {
            if (value >= 0) {
                return { UnsignedInteger: value };
            } else {
                return { Integer: value };
            }
        } else {
            return { Float: value };
        }
    } else if (typeof value === 'boolean') {
        return { Boolean: value };
    } else if (value instanceof Uint8Array) {
        return { Buffer: Array.from(value) };
    } else if (value instanceof Vector) {
        return { Vector: value.vector };
    } else if (Array.isArray(value)) {
        return { List: value.map((item) => encode(item)) };
    } else if (value instanceof LazyStringMap) {
        return { LazyStringMap: value.map };
    } else if (value instanceof Date) {
        return { Timestamptz: value.toISOString() };
    } else if (value instanceof Interval) {
        return { Interval: value.milliseconds };
    } else if (value instanceof TimeZone) {
        return { TimeZone: value.timezone };
    } else if (value instanceof Map) {
        const mapEntries: [KhronosValue, KhronosValue][] = [];
        for (const [key, val] of value.entries()) {
            mapEntries.push([encode(key), encode(val)]);
        }
        return { Map: mapEntries };
    } else if (typeof value === 'object') {
        const mapEntries: [KhronosValue, KhronosValue][] = [];
        for (const [key, val] of Object.entries(value)) {
            // Objects only allow string keys
            mapEntries.push([{ Text: key }, encode(val)]);
        }
        return { Map: mapEntries };
    } else {
        // Fallback to string representation
        return { Text: value.toString() };
    }
}

const json: KhronosValue = {
  "Map": [
    [
      {
        "Text": "key1"
      },
      {
        "Text": "value1"
      }
    ],
    [
      {
        "Text": "key2"
      },
      {
        "Vector": [
          1.0,
          2.1,
          3.2
        ]
      }
    ],
    [
      {
        "Text": "key3"
      },
      {
        "LazyStringMap": {}
      }
    ]
  ]
}


console.log(decode(json));
console.log(encode(decode(json)));