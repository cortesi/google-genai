use derive_setters::*;
use serde_derive::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use time::Date;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Outcome {
    #[default]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmCategory {
    #[default]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {
    #[default]
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct VideoMetadata {
    pub end_offset: Option<String>,
    pub start_offset: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters)]
#[setters(strip_option, into)]
pub struct CodeExecutionResult {
    pub outcome: Outcome,
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct FunctionCall {
    pub id: Option<String>,
    pub args: Option<serde_json::Value>,
    pub name: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct FunctionResponse {
    pub id: Option<String>,
    pub name: String,
    pub response: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blob {
    pub data: Vec<u8>,
    pub mime_type: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct Part {
    pub video_metadata: Option<VideoMetadata>,
    pub code_execution_result: Option<CodeExecutionResult>,
    pub executable_code: Option<ExecutableCode>,
    pub file_data: Option<FileData>,
    pub function_call: Option<FunctionCall>,
    pub function_response: Option<FunctionResponse>,
    pub inline_data: Option<Blob>,
    pub text: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct Content {
    pub parts: Option<Vec<Part>>,
    pub role: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
/// Schema that defines the format of input and output data.
/// Represents a select subset of an OpenAPI 3.0 schema object.
pub struct Schema {
    /// Optional. Minimum number of elements for Type.ARRAY.
    pub min_items: Option<String>,
    /// Optional. Example of the object.
    pub example: Option<serde_json::Value>,
    /// Optional. The order of the properties.
    pub property_ordering: Option<Vec<String>>,
    /// Optional. Pattern of the Type.STRING to restrict a string to a regular expression.
    pub pattern: Option<String>,
    /// Optional. Minimum value of the Type.INTEGER and Type.NUMBER.
    pub minimum: Option<f64>,
    /// Optional. Default value of the data.
    pub default: Option<serde_json::Value>,
    /// Optional. The value should be validated against any of the subschemas.
    pub any_of: Option<Vec<Schema>>,
    /// Optional. Maximum length of the Type.STRING.
    pub max_length: Option<String>,
    /// Optional. The title of the Schema.
    pub title: Option<String>,
    /// Optional. Minimum length of the Type.STRING.
    pub min_length: Option<String>,
    /// Optional. Minimum number of properties for Type.OBJECT.
    pub min_properties: Option<String>,
    /// Optional. Maximum number of elements for Type.ARRAY.
    pub max_items: Option<String>,
    /// Optional. Maximum value of the Type.INTEGER and Type.NUMBER.
    pub maximum: Option<f64>,
    /// Optional. Indicates if the value may be null.
    pub nullable: Option<bool>,
    /// Optional. Maximum number of properties for Type.OBJECT.
    pub max_properties: Option<String>,
    /// Optional. The type of the data.
    #[setters(skip)]
    pub r#type: Option<Type>,
    /// Optional. The description of the data.
    pub description: Option<String>,
    /// Optional. Possible values of the element of primitive type with enum format.
    #[setters(skip)]
    pub r#enum: Option<Vec<String>>,
    /// Optional. The format of the data.
    pub format: Option<String>,
    /// Optional. Schema of the elements of Type.ARRAY.
    pub items: Option<Box<Schema>>,
    /// Optional. Properties of Type.OBJECT.
    pub properties: Option<std::collections::HashMap<String, Schema>>,
    /// Optional. Required properties of Type.OBJECT.
    pub required: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters)]
#[setters(strip_option, into)]
/// Safety settings.
pub struct SafetySetting {
    /// Determines if the harm block method uses probability or severity scores.
    pub method: Option<HarmBlockMethod>,
    /// Required. Harm category.
    pub category: HarmCategory,
    /// Required. The harm block threshold.
    pub threshold: HarmBlockThreshold,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
/// Defines a function that the model can generate JSON inputs for.
pub struct FunctionDeclaration {
    /// Describes the output from the function in the OpenAPI JSON Schema Object format.
    pub response: Option<Schema>,
    /// Optional. Description and purpose of the function.
    pub description: Option<String>,
    /// Required. The name of the function to call.
    pub name: String,
    /// Optional. Describes the parameters to this function in JSON Schema Object format.
    pub parameters: Option<Schema>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleSearch {}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
/// Describes the options to customize dynamic retrieval.
pub struct DynamicRetrievalConfig {
    /// The mode of the predictor to be used in dynamic retrieval.
    pub mode: Option<DynamicRetrievalConfigMode>,
    /// Optional. The threshold to be used in dynamic retrieval.
    pub dynamic_threshold: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Tool to retrieve public web data for grounding, powered by Google.
pub struct GoogleSearchRetrieval {
    /// Specifies the dynamic retrieval configuration for the given source.
    pub dynamic_retrieval_config: Option<DynamicRetrievalConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Retrieve from Vertex AI Search datastore for grounding.
pub struct VertexAISearch {
    /// Required. Fully-qualified Vertex AI Search data store resource ID.
    pub datastore: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
/// The definition of the RAG resource.
pub struct VertexRAGStoreRAGResource {
    /// Optional. RAGCorpora resource name.
    pub rag_corpus: Option<String>,
    /// Optional. rag_file_id. The files should be in the same rag_corpus.
    pub rag_file_ids: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct VertexRAGStore {
    pub rag_corpora: Option<Vec<String>>,
    pub rag_resources: Option<Vec<VertexRAGStoreRAGResource>>,
    pub similarity_top_k: Option<i64>,
    pub vector_distance_threshold: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct Retrieval {
    pub vertex_ai_search: Option<VertexAISearch>,
    pub vertex_rag_store: Option<VertexRAGStore>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCodeExecution {}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
/// Tool details of a tool that the model may use to generate a response.
pub struct Tool {
    /// List of function declarations that the tool supports.
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
    /// Optional. Retrieval tool type.
    pub retrieval: Option<Retrieval>,
    /// Optional. Google Search tool type.
    pub google_search: Option<GoogleSearch>,
    /// Optional. GoogleSearchRetrieval tool type.
    pub google_search_retrieval: Option<GoogleSearchRetrieval>,
    /// Optional. CodeExecution tool type.
    pub code_execution: Option<ToolCodeExecution>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct FunctionCallingConfig {
    pub mode: Option<FunctionCallingConfigMode>,
    pub allowed_function_names: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ToolConfig {
    pub function_calling_config: Option<FunctionCallingConfig>,
}

#[skip_serializing_none]
/// The configuration for the prebuilt speaker to use.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PrebuiltVoiceConfig {
    /// The name of the prebuilt voice to use.
    pub voice_name: Option<String>,
}

#[skip_serializing_none]
/// The configuration for the voice to use.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VoiceConfig {
    /// The configuration for the speaker to use.
    pub prebuilt_voice_config: Option<PrebuiltVoiceConfig>,
}

#[skip_serializing_none]
/// The speech generation configuration.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SpeechConfig {
    /// The configuration for the speaker to use.
    pub voice_config: Option<VoiceConfig>,
}

#[skip_serializing_none]
/// When automated routing is specified, the routing will be determined by the pretrained routing model.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GenerationConfigRoutingConfigAutoRoutingMode {
    /// The model routing preference.
    pub model_routing_preference: Option<String>,
}

#[skip_serializing_none]
/// When manual routing is set, the specified model will be used directly.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerationConfigRoutingConfigManualRoutingMode {
    /// The model name to use. Only the public LLM models are accepted.
    pub model_name: Option<String>,
}

#[skip_serializing_none]
/// The configuration for routing the request to a specific model.
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct GenerationConfigRoutingConfig {
    /// Automated routing.
    pub auto_mode: Option<GenerationConfigRoutingConfigAutoRoutingMode>,
    /// Manual routing.
    pub manual_mode: Option<GenerationConfigRoutingConfigManualRoutingMode>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default, Setters)]
#[setters(strip_option, into)]
pub struct GenerateContentParameters {
    pub model: Option<String>,
    pub contents: Option<Vec<Content>>,
    pub config: Option<GenerateContentConfig>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct Citation {
    pub end_index: Option<i64>,
    pub license: Option<String>,
    pub publication_date: Option<Date>,
    pub start_index: Option<i64>,
    pub title: Option<String>,
    pub uri: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CitationMetadata {
    pub citations: Option<Vec<Citation>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct GroundingChunkRetrievedContext {
    pub text: Option<String>,
    pub title: Option<String>,
    pub uri: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct GroundingChunkWeb {
    pub title: Option<String>,
    pub uri: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct GroundingChunk {
    pub retrieved_context: Option<GroundingChunkRetrievedContext>,
    pub web: Option<GroundingChunkWeb>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct Segment {
    pub end_index: Option<i64>,
    pub part_index: Option<i64>,
    pub start_index: Option<i64>,
    pub text: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct GroundingSupport {
    pub confidence_scores: Option<Vec<f64>>,
    pub grounding_chunk_indices: Option<Vec<i64>>,
    pub segment: Option<Segment>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RetrievalMetadata {
    pub google_search_dynamic_retrieval_score: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct SearchEntryPoint {
    pub rendered_content: Option<String>,
    pub sdk_blob: Option<Vec<u8>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct GroundingMetadata {
    pub grounding_chunks: Option<Vec<GroundingChunk>>,
    pub grounding_supports: Option<Vec<GroundingSupport>>,
    pub retrieval_metadata: Option<RetrievalMetadata>,
    pub retrieval_queries: Option<Vec<String>>,
    pub search_entry_point: Option<SearchEntryPoint>,
    pub web_search_queries: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct LogprobsResultCandidate {
    pub log_probability: Option<f64>,
    pub token: Option<String>,
    pub token_id: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LogprobsResultTopCandidates {
    pub candidates: Option<Vec<LogprobsResultCandidate>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct LogprobsResult {
    pub chosen_candidates: Option<Vec<LogprobsResultCandidate>>,
    pub top_candidates: Option<Vec<LogprobsResultTopCandidates>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct SafetyRating {
    pub blocked: Option<bool>,
    pub category: Option<HarmCategory>,
    pub probability: Option<HarmProbability>,
    pub probability_score: Option<f64>,
    pub severity: Option<HarmSeverity>,
    pub severity_score: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct Candidate {
    pub content: Option<Content>,
    pub citation_metadata: Option<CitationMetadata>,
    pub finish_message: Option<String>,
    pub token_count: Option<i64>,
    pub avg_logprobs: Option<f64>,
    pub finish_reason: Option<FinishReason>,
    pub grounding_metadata: Option<GroundingMetadata>,
    pub index: Option<i64>,
    pub logprobs_result: Option<LogprobsResult>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct GenerateContentResponsePromptFeedback {
    pub block_reason: Option<BlockedReason>,
    pub block_reason_message: Option<String>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct GenerateContentResponseUsageMetadata {
    pub cached_content_token_count: Option<i64>,
    pub candidates_token_count: Option<i64>,
    pub prompt_token_count: Option<i64>,
    pub total_token_count: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct GenerateContentResponse {
    pub candidates: Option<Vec<Candidate>>,
    pub model_version: Option<String>,
    pub prompt_feedback: Option<GenerateContentResponsePromptFeedback>,
    pub usage_metadata: Option<GenerateContentResponseUsageMetadata>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct GenerateContentConfig {
    pub system_instruction: Option<Content>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<f64>,
    pub candidate_count: Option<i64>,
    pub max_output_tokens: Option<i64>,
    pub stop_sequences: Option<Vec<String>>,
    pub response_logprobs: Option<bool>,
    pub logprobs: Option<i64>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub seed: Option<i64>,
    pub response_mime_type: Option<String>,
    pub response_schema: Option<Schema>,
    pub routing_config: Option<GenerationConfigRoutingConfig>,
    pub safety_settings: Option<Vec<SafetySetting>>,
    pub tools: Option<Vec<Tool>>,
    pub tool_config: Option<ToolConfig>,
    pub cached_content: Option<String>,
    pub response_modalities: Option<Vec<String>>,
    pub media_resolution: Option<MediaResolution>,
    pub speech_config: Option<SpeechConfig>,
}

/// Configuration for generation settings.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct GenerationConfig {
    /// Optional. If enabled, audio timestamp will be included.
    pub audio_timestamp: Option<bool>,
    /// Optional. Number of candidates to generate.
    pub candidate_count: Option<i64>,
    /// Optional. Frequency penalties.
    pub frequency_penalty: Option<f64>,
    /// Optional. Logit probabilities.
    pub logprobs: Option<i64>,
    /// Optional. The maximum number of output tokens to generate per message.
    pub max_output_tokens: Option<i64>,
    /// Optional. Positive penalties.
    pub presence_penalty: Option<f64>,
    /// Optional. If true, export the logprobs results in response.
    pub response_logprobs: Option<bool>,
    /// Optional. Output response MIME type of the generated candidate text.
    pub response_mime_type: Option<String>,
    /// Optional. Schema object allows the definition of input and output data types.
    pub response_schema: Option<Schema>,
    /// Optional. Routing configuration.
    pub routing_config: Option<GenerationConfigRoutingConfig>,
    /// Optional. Seed.
    pub seed: Option<i64>,
    /// Optional. Stop sequences.
    pub stop_sequences: Option<Vec<String>>,
    /// Optional. Controls the randomness of predictions.
    pub temperature: Option<f64>,
    /// Optional. If specified, top-k sampling will be used.
    pub top_k: Option<f64>,
    /// Optional. If specified, nucleus sampling will be used.
    pub top_p: Option<f64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Setters, Default)]
#[setters(strip_option, into)]
pub struct GenerateContentReq {
    pub model: String,
    pub contents: Vec<Content>,
    #[serde(rename = "generationConfig")]
    pub generation_config: Option<GenerationConfig>,
    pub system_instruction: Option<Content>,
}
