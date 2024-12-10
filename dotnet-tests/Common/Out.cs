using System.Text.Json;

namespace Common;


public interface IOutMessage;

public class LogMessage : IOutMessage
{
    public string? Message { get; set; }
}

public class ErrorMessage : IOutMessage
{
    public string? Message { get; set; }
}

public class ReadyMessage : IOutMessage { }

public enum OutMessageType
{
    Log,
    Ready,
    Payload,
    Error,
}

public class GeneralMessage : IOutMessage
{
    public JsonDocument? Payload { get; set; }
}

class OutMessageConverter : TaggedUnionConverter<IOutMessage, OutMessageType>
{
    protected override string TagName => "type";

    protected override IOutMessage? FromEnum(JsonDocument document, JsonSerializerOptions options, OutMessageType type)
    {
        return type switch
        {
            OutMessageType.Log => document.Deserialize<LogMessage>(options),
            OutMessageType.Ready => document.Deserialize<ReadyMessage>(options),
            OutMessageType.Payload => document.Deserialize<GeneralMessage>(options),
            OutMessageType.Error => document.Deserialize<ErrorMessage>(options),
            _ => throw new JsonException("Unknown type variant")
        };
    }

    protected override OutMessageType? ParseEnum(string value)
    {
        return value switch
        {
            "log" => OutMessageType.Log,
            "ready" => OutMessageType.Ready,
            "payload" => OutMessageType.Payload,
            "error" => OutMessageType.Error,
            _ => null
        };
    }
}