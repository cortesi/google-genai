use serde_derive::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use time::Date;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Outcome {
    Unspecified,
    Ok,
    Failed,
    DeadlineExceeded,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Language {
    Unspecified,
    Python,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    Unspecified,
    String,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmCategory {
    Unspecified,
    HateSpeech,
    DangerousContent,
    Harassment,
    SexuallyExplicit,
    CivicIntegrity,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HarmBlockMethod {
    #[serde(rename = "HARM_BLOCK_METHOD_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "SEVERITY")]
    Severity,
    #[serde(rename = "PROBABILITY")]
    Probability,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {
    Unspecified,
    BlockLowAndAbove,
    BlockMediumAndAbove,
    BlockOnlyHigh,
    BlockNone,
    Off,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Mode {
    Unspecified,
    Dynamic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinishReason {
    Unspecified,
    Stop,
    MaxTokens,
    Safety,
    Recitation,
    Other,
    Blocklist,
    ProhibitedContent,
    Spii,
    MalformedFunctionCall,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HarmProbability {
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "NEGLIGIBLE")]
    Negligible,
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "HIGH")]
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmSeverity {
    Unspecified,
    HarmSeverityNegligible,
    HarmSeverityLow,
    HarmSeverityMedium,
    HarmSeverityHigh,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockedReason {
    Unspecified,
    Safety,
    Other,
    Blocklist,
    ProhibitedContent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DynamicRetrievalConfigMode {
    Unspecified,
    ModeDynamic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FunctionCallingConfigMode {
    #[serde(rename = "MODE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "NONE")]
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaResolution {
    Unspecified,
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaskReferenceMode {
    MaskModeDefault,
    MaskModeUserProvided,
    MaskModeBackground,
    MaskModeForeground,
    MaskModeSemantic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ControlReferenceType {
    ControlTypeDefault,
    ControlTypeCanny,
    ControlTypeScribble,
    ControlTypeFaceMesh,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubjectReferenceType {
    SubjectTypeDefault,
    SubjectTypePerson,
    SubjectTypeAnimal,
    SubjectTypeProduct,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_offset: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeExecutionResult {
    pub outcome: Outcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutableCode {
    pub code: String,
    pub language: Language,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileData {
    pub file_uri: String,
    pub mime_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub response: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blob {
    pub data: Vec<u8>,
    pub mime_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_execution_result: Option<CodeExecutionResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable_code: Option<ExecutableCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<FileData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response: Option<FunctionResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_data: Option<Blob>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Part>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_ordering: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafetySetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<HarmBlockMethod>,
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionDeclaration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Schema>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleSearch {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicRetrievalConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<DynamicRetrievalConfigMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_threshold: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleSearchRetrieval {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_retrieval_config: Option<DynamicRetrievalConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VertexAISearch {
    pub datastore: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VertexRAGStoreRAGResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rag_corpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rag_file_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VertexRAGStore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rag_corpora: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rag_resources: Option<Vec<VertexRAGStoreRAGResource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_top_k: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_distance_threshold: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Retrieval {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertex_ai_search: Option<VertexAISearch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertex_rag_store: Option<VertexRAGStore>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCodeExecution {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval: Option<Retrieval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_search: Option<GoogleSearch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_search_retrieval: Option<GoogleSearchRetrieval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_execution: Option<ToolCodeExecution>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCallingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<FunctionCallingConfigMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_function_names: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_calling_config: Option<FunctionCallingConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrebuiltVoiceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoiceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prebuilt_voice_config: Option<PrebuiltVoiceConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeechConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_config: Option<VoiceConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerationConfigRoutingConfigAutoRoutingMode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_routing_preference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerationConfigRoutingConfigManualRoutingMode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerationConfigRoutingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_mode: Option<GenerationConfigRoutingConfigAutoRoutingMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_mode: Option<GenerationConfigRoutingConfigManualRoutingMode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateContentConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_schema: Option<Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<GenerationConfigRoutingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_modalities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_resolution: Option<MediaResolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_config: Option<SpeechConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateContentParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<Content>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GenerateContentConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Citation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
