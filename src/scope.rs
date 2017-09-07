

use std::str::FromStr;

impl FromStr for Scope {
    type Err = String;
    fn from_str(url: &str) -> Result<Scope, String> {
      Scope::scope(url).ok_or(format!("invalid scope {}", url))
    }
}

// fixme: try this with a macro and a set of scope urls

/// API Scope values
///
/// For convenience, `std::str::FromStr` is defined for
/// enabling cases where you have the url representing the scope
/// and which to call url.parse() to resolve a `Scope` value
pub enum Scope {
    /// Scope for https://www.googleapis.com/auth/activity
    Activity,
    /// Scope for https://www.googleapis.com/auth/adexchange.buyer
    AdexchangeBuyer,
    /// Scope for https://www.googleapis.com/auth/adexchange.seller
    AdexchangeSeller,
    /// Scope for https://www.googleapis.com/auth/adexchange.seller.readonly
    AdexchangeSellerReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.datatransfer
    AdminDatatransfer,
    /// Scope for https://www.googleapis.com/auth/admin.datatransfer.readonly
    AdminDatatransferReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.directory.customer
    AdminDirectoryCustomer,
    /// Scope for https://www.googleapis.com/auth/admin.directory.customer.readonly
    AdminDirectoryCustomerReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.directory.device.chromeos
    AdminDirectoryDeviceChromeos,
    /// Scope for https://www.googleapis.com/auth/admin.directory.device.chromeos.readonly
    AdminDirectoryDeviceChromeosReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.directory.device.mobile
    AdminDirectoryDeviceMobile,
    /// Scope for https://www.googleapis.com/auth/admin.directory.device.mobile.action
    AdminDirectoryDeviceMobileAction,
    /// Scope for https://www.googleapis.com/auth/admin.directory.device.mobile.readonly
    AdminDirectoryDeviceMobileReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.directory.domain
    AdminDirectoryDomain,
    /// Scope for https://www.googleapis.com/auth/admin.directory.domain.readonly
    AdminDirectoryDomainReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.directory.group
    AdminDirectoryGroup,
    /// Scope for https://www.googleapis.com/auth/admin.directory.group.member
    AdminDirectoryGroupMember,
    /// Scope for https://www.googleapis.com/auth/admin.directory.group.member.readonly
    AdminDirectoryGroupMemberReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.directory.group.readonly
    AdminDirectoryGroupReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.directory.notifications
    AdminDirectoryNotifications,
    /// Scope for https://www.googleapis.com/auth/admin.directory.orgunit
    AdminDirectoryOrgunit,
    /// Scope for https://www.googleapis.com/auth/admin.directory.orgunit.readonly
    AdminDirectoryOrgunitReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.directory.resource.calendar
    AdminDirectoryResourceCalendar,
    /// Scope for https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly
    AdminDirectoryResourceCalendarReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.directory.rolemanagement
    AdminDirectoryRolemanagement,
    /// Scope for https://www.googleapis.com/auth/admin.directory.rolemanagement.readonly
    AdminDirectoryRolemanagementReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.directory.user
    AdminDirectoryUser,
    /// Scope for https://www.googleapis.com/auth/admin.directory.user.alias
    AdminDirectoryUserAlias,
    /// Scope for https://www.googleapis.com/auth/admin.directory.user.alias.readonly
    AdminDirectoryUserAliasReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.directory.user.readonly
    AdminDirectoryUserReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.directory.user.security
    AdminDirectoryUserSecurity,
    /// Scope for https://www.googleapis.com/auth/admin.directory.userschema
    AdminDirectoryUserschema,
    /// Scope for https://www.googleapis.com/auth/admin.directory.userschema.readonly
    AdminDirectoryUserschemaReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.reports.audit.readonly
    AdminReportsAuditReadonly,
    /// Scope for https://www.googleapis.com/auth/admin.reports.usage.readonly
    AdminReportsUsageReadonly,
    /// Scope for https://www.googleapis.com/auth/adsense
    Adsense,
    /// Scope for https://www.googleapis.com/auth/adsense.readonly
    AdsenseReadonly,
    /// Scope for https://www.googleapis.com/auth/adsensehost
    Adsensehost,
    /// Scope for https://www.googleapis.com/auth/analytics
    Analytics,
    /// Scope for https://www.googleapis.com/auth/analytics.edit
    AnalyticsEdit,
    /// Scope for https://www.googleapis.com/auth/analytics.manage.users
    AnalyticsManageUsers,
    /// Scope for https://www.googleapis.com/auth/analytics.manage.users.readonly
    AnalyticsManageUsersReadonly,
    /// Scope for https://www.googleapis.com/auth/analytics.provision
    AnalyticsProvision,
    /// Scope for https://www.googleapis.com/auth/analytics.readonly
    AnalyticsReadonly,
    /// Scope for https://www.googleapis.com/auth/androidenterprise
    Androidenterprise,
    /// Scope for https://www.googleapis.com/auth/androidpublisher
    Androidpublisher,
    /// Scope for https://www.googleapis.com/auth/appengine.admin
    AppengineAdmin,
    /// Scope for https://www.googleapis.com/auth/apps.groups.migration
    AppsGroupsMigration,
    /// Scope for https://www.googleapis.com/auth/apps.groups.settings
    AppsGroupsSettings,
    /// Scope for https://www.googleapis.com/auth/apps.licensing
    AppsLicensing,
    /// Scope for https://www.googleapis.com/auth/apps.order
    AppsOrder,
    /// Scope for https://www.googleapis.com/auth/apps.order.readonly
    AppsOrderReadonly,
    /// Scope for https://www.googleapis.com/auth/appstate
    Appstate,
    /// Scope for https://www.googleapis.com/auth/bigquery
    Bigquery,
    /// Scope for https://www.googleapis.com/auth/bigquery.insertdata
    BigqueryInsertdata,
    /// Scope for https://www.googleapis.com/auth/blogger
    Blogger,
    /// Scope for https://www.googleapis.com/auth/blogger.readonly
    BloggerReadonly,
    /// Scope for https://www.googleapis.com/auth/books
    Books,
    /// Scope for https://www.googleapis.com/auth/calendar
    Calendar,
    /// Scope for https://www.google.com/calendar/feeds
    CalendarFeeds,
    /// Scope for https://www.googleapis.com/auth/calendar.readonly
    CalendarReadonly,
    /// Scope for https://www.googleapis.com/auth/classroom.courses
    ClassroomCourses,
    /// Scope for https://www.googleapis.com/auth/classroom.courses.readonly
    ClassroomCoursesReadonly,
    /// Scope for https://www.googleapis.com/auth/classroom.coursework.me
    ClassroomCourseworkMe,
    /// Scope for https://www.googleapis.com/auth/classroom.coursework.me.readonly
    ClassroomCourseworkMeReadonly,
    /// Scope for https://www.googleapis.com/auth/classroom.coursework.students
    ClassroomCourseworkStudents,
    /// Scope for https://www.googleapis.com/auth/classroom.coursework.students.readonly
    ClassroomCourseworkStudentsReadonly,
    /// Scope for https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly
    ClassroomGuardianlinksMeReadonly,
    /// Scope for https://www.googleapis.com/auth/classroom.guardianlinks.students
    ClassroomGuardianlinksStudents,
    /// Scope for https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly
    ClassroomGuardianlinksStudentsReadonly,
    /// Scope for https://www.googleapis.com/auth/classroom.profile.emails
    ClassroomProfileEmails,
    /// Scope for https://www.googleapis.com/auth/classroom.profile.photos
    ClassroomProfilePhotos,
    /// Scope for https://www.googleapis.com/auth/classroom.rosters
    ClassroomRosters,
    /// Scope for https://www.googleapis.com/auth/classroom.rosters.readonly
    ClassroomRostersReadonly,
    /// Scope for https://www.googleapis.com/auth/classroom.student-submissions.me.readonly
    ClassroomStudentSubmissionsMeReadonly,
    /// Scope for https://www.googleapis.com/auth/classroom.student-submissions.students.readonly
    ClassroomStudentSubmissionsStudentsReadonly,
    /// Scope for https://www.googleapis.com/auth/cloud_debugger
    CloudDebugger,
    /// Scope for https://www.googleapis.com/auth/cloud-language
    CloudLanguage,
    /// Scope for https://www.googleapis.com/auth/cloud-platform
    CloudPlatform,
    /// Scope for https://www.googleapis.com/auth/cloud-platform.read-only
    CloudPlatformReadOnly,
    /// Scope for https://www.googleapis.com/auth/cloud-translation
    CloudTranslation,
    /// Scope for https://www.googleapis.com/auth/cloud.useraccounts
    CloudUseraccounts,
    /// Scope for https://www.googleapis.com/auth/cloud.useraccounts.readonly
    CloudUseraccountsReadonly,
    /// Scope for https://www.googleapis.com/auth/cloudruntimeconfig
    Cloudruntimeconfig,
    /// Scope for https://www.googleapis.com/auth/compute
    Compute,
    /// Scope for https://www.googleapis.com/auth/compute.readonly
    ComputeReadonly,
    /// Scope for https://www.googleapis.com/auth/contacts
    Contacts,
    /// Scope for https://www.googleapis.com/auth/contacts.readonly
    ContactsReadonly,
    /// Scope for https://www.googleapis.com/auth/content
    Content,
    /// Scope for https://www.googleapis.com/auth/datastore
    Datastore,
    /// Scope for https://www.googleapis.com/auth/ddmconversions
    Ddmconversions,
    /// Scope for https://www.googleapis.com/auth/devstorage.full_control
    DevstorageFullControl,
    /// Scope for https://www.googleapis.com/auth/devstorage.read_only
    DevstorageReadOnly,
    /// Scope for https://www.googleapis.com/auth/devstorage.read_write
    DevstorageReadWrite,
    /// Scope for https://www.googleapis.com/auth/dfareporting
    Dfareporting,
    /// Scope for https://www.googleapis.com/auth/dfatrafficking
    Dfatrafficking,
    /// Scope for https://www.googleapis.com/auth/doubleclickbidmanager
    Doubleclickbidmanager,
    /// Scope for https://www.googleapis.com/auth/doubleclicksearch
    Doubleclicksearch,
    /// Scope for https://www.googleapis.com/auth/drive
    Drive,
    /// Scope for https://www.googleapis.com/auth/drive.appdata
    DriveAppdata,
    /// Scope for https://www.googleapis.com/auth/drive.file
    DriveFile,
    /// Scope for https://www.googleapis.com/auth/drive.metadata
    DriveMetadata,
    /// Scope for https://www.googleapis.com/auth/drive.metadata.readonly
    DriveMetadataReadonly,
    /// Scope for https://www.googleapis.com/auth/drive.photos.readonly
    DrivePhotosReadonly,
    /// Scope for https://www.googleapis.com/auth/drive.readonly
    DriveReadonly,
    /// Scope for https://www.googleapis.com/auth/drive.scripts
    DriveScripts,
    /// Scope for https://www.googleapis.com/auth/firebase
    Firebase,
    /// Scope for https://www.googleapis.com/auth/firebase.readonly
    FirebaseReadonly,
    /// Scope for https://www.googleapis.com/auth/fitness.activity.read
    FitnessActivityRead,
    /// Scope for https://www.googleapis.com/auth/fitness.activity.write
    FitnessActivityWrite,
    /// Scope for https://www.googleapis.com/auth/fitness.blood_glucose.read
    FitnessBloodGlucoseRead,
    /// Scope for https://www.googleapis.com/auth/fitness.blood_glucose.write
    FitnessBloodGlucoseWrite,
    /// Scope for https://www.googleapis.com/auth/fitness.blood_pressure.read
    FitnessBloodPressureRead,
    /// Scope for https://www.googleapis.com/auth/fitness.blood_pressure.write
    FitnessBloodPressureWrite,
    /// Scope for https://www.googleapis.com/auth/fitness.body.read
    FitnessBodyRead,
    /// Scope for https://www.googleapis.com/auth/fitness.body_temperature.read
    FitnessBodyTemperatureRead,
    /// Scope for https://www.googleapis.com/auth/fitness.body_temperature.write
    FitnessBodyTemperatureWrite,
    /// Scope for https://www.googleapis.com/auth/fitness.body.write
    FitnessBodyWrite,
    /// Scope for https://www.googleapis.com/auth/fitness.location.read
    FitnessLocationRead,
    /// Scope for https://www.googleapis.com/auth/fitness.location.write
    FitnessLocationWrite,
    /// Scope for https://www.googleapis.com/auth/fitness.nutrition.read
    FitnessNutritionRead,
    /// Scope for https://www.googleapis.com/auth/fitness.nutrition.write
    FitnessNutritionWrite,
    /// Scope for https://www.googleapis.com/auth/fitness.oxygen_saturation.read
    FitnessOxygenSaturationRead,
    /// Scope for https://www.googleapis.com/auth/fitness.oxygen_saturation.write
    FitnessOxygenSaturationWrite,
    /// Scope for https://www.googleapis.com/auth/fitness.reproductive_health.read
    FitnessReproductiveHealthRead,
    /// Scope for https://www.googleapis.com/auth/fitness.reproductive_health.write
    FitnessReproductiveHealthWrite,
    /// Scope for https://www.googleapis.com/auth/forms
    Forms,
    /// Scope for https://www.googleapis.com/auth/forms.currentonly
    FormsCurrentonly,
    /// Scope for https://www.googleapis.com/auth/fusiontables
    Fusiontables,
    /// Scope for https://www.googleapis.com/auth/fusiontables.readonly
    FusiontablesReadonly,
    /// Scope for https://www.googleapis.com/auth/games
    Games,
    /// Scope for https://www.googleapis.com/auth/genomics
    Genomics,
    /// Scope for https://www.googleapis.com/auth/genomics.readonly
    GenomicsReadonly,
    /// Scope for https://www.googleapis.com/auth/glass.location
    GlassLocation,
    /// Scope for https://www.googleapis.com/auth/glass.timeline
    GlassTimeline,
    /// Scope for https://mail.google.com/
    Gmail,
    /// Scope for https://www.googleapis.com/auth/gmail.compose
    GmailCompose,
    /// Scope for https://www.googleapis.com/auth/gmail.insert
    GmailInsert,
    /// Scope for https://www.googleapis.com/auth/gmail.labels
    GmailLabels,
    /// Scope for https://www.googleapis.com/auth/gmail.metadata
    GmailMetadata,
    /// Scope for https://www.googleapis.com/auth/gmail.modify
    GmailModify,
    /// Scope for https://www.googleapis.com/auth/gmail.readonly
    GmailReadonly,
    /// Scope for https://www.googleapis.com/auth/gmail.send
    GmailSend,
    /// Scope for https://www.googleapis.com/auth/gmail.settings.basic
    GmailSettingsBasic,
    /// Scope for https://www.googleapis.com/auth/gmail.settings.sharing
    GmailSettingsSharing,
    /// Scope for https://www.googleapis.com/auth/groups
    Groups,
    /// Scope for https://www.googleapis.com/auth/logging.admin
    LoggingAdmin,
    /// Scope for https://www.googleapis.com/auth/logging.read
    LoggingRead,
    /// Scope for https://www.googleapis.com/auth/logging.write
    LoggingWrite,
    /// Scope for https://www.google.com/m8/feeds
    M8Feeds,
    /// Scope for https://www.googleapis.com/auth/manufacturercenter
    Manufacturercenter,
    /// Scope for https://www.googleapis.com/auth/monitoring
    Monitoring,
    /// Scope for https://www.googleapis.com/auth/monitoring.read
    MonitoringRead,
    /// Scope for https://www.googleapis.com/auth/monitoring.write
    MonitoringWrite,
    /// Scope for https://www.googleapis.com/auth/ndev.clouddns.readonly
    NdevClouddnsReadonly,
    /// Scope for https://www.googleapis.com/auth/ndev.clouddns.readwrite
    NdevClouddnsReadwrite,
    /// Scope for https://www.googleapis.com/auth/ndev.cloudman
    NdevCloudman,
    /// Scope for https://www.googleapis.com/auth/ndev.cloudman.readonly
    NdevCloudmanReadonly,
    /// Scope for https://www.googleapis.com/auth/playmovies_partner.readonly
    PlaymoviesPartnerReadonly,
    /// Scope for https://www.googleapis.com/auth/plus.circles.read
    PlusCirclesRead,
    /// Scope for https://www.googleapis.com/auth/plus.circles.write
    PlusCirclesWrite,
    /// Scope for https://www.googleapis.com/auth/plus.login
    PlusLogin,
    /// Scope for https://www.googleapis.com/auth/plus.me
    PlusMe,
    /// Scope for https://www.googleapis.com/auth/plus.media.upload
    PlusMediaUpload,
    /// Scope for https://www.googleapis.com/auth/plus.profiles.read
    PlusProfilesRead,
    /// Scope for https://www.googleapis.com/auth/plus.stream.read
    PlusStreamRead,
    /// Scope for https://www.googleapis.com/auth/plus.stream.write
    PlusStreamWrite,
    /// Scope for https://www.googleapis.com/auth/prediction
    Prediction,
    /// Scope for https://www.googleapis.com/auth/presentations
    Presentations,
    /// Scope for https://www.googleapis.com/auth/presentations.readonly
    PresentationsReadonly,
    /// Scope for https://www.googleapis.com/auth/pubsub
    Pubsub,
    /// Scope for https://www.googleapis.com/auth/replicapool
    Replicapool,
    /// Scope for https://www.googleapis.com/auth/replicapool.readonly
    ReplicapoolReadonly,
    /// Scope for https://www.googleapis.com/auth/service.management
    ServiceManagement,
    /// Scope for https://www.googleapis.com/auth/service.management.readonly
    ServiceManagementReadonly,
    /// Scope for https://www.googleapis.com/auth/servicecontrol
    Servicecontrol,
    /// Scope for https://www.googleapis.com/auth/siteverification
    Siteverification,
    /// Scope for https://www.googleapis.com/auth/siteverification.verify_only
    SiteverificationVerifyOnly,
    /// Scope for https://www.googleapis.com/auth/source.read_only
    SourceReadOnly,
    /// Scope for https://www.googleapis.com/auth/source.read_write
    SourceReadWrite,
    /// Scope for https://www.googleapis.com/auth/spanner.admin
    SpannerAdmin,
    /// Scope for https://www.googleapis.com/auth/spanner.data
    SpannerData,
    /// Scope for https://www.googleapis.com/auth/spreadsheets
    Spreadsheets,
    /// Scope for https://www.googleapis.com/auth/spreadsheets.readonly
    SpreadsheetsReadonly,
    /// Scope for https://www.googleapis.com/auth/sqlservice.admin
    SqlserviceAdmin,
    /// Scope for https://www.googleapis.com/auth/tagmanager.delete.containers
    TagmanagerDeleteContainers,
    /// Scope for https://www.googleapis.com/auth/tagmanager.edit.containers
    TagmanagerEditContainers,
    /// Scope for https://www.googleapis.com/auth/tagmanager.edit.containerversions
    TagmanagerEditContainerversions,
    /// Scope for https://www.googleapis.com/auth/tagmanager.manage.accounts
    TagmanagerManageAccounts,
    /// Scope for https://www.googleapis.com/auth/tagmanager.manage.users
    TagmanagerManageUsers,
    /// Scope for https://www.googleapis.com/auth/tagmanager.publish
    TagmanagerPublish,
    /// Scope for https://www.googleapis.com/auth/tagmanager.readonly
    TagmanagerReadonly,
    /// Scope for https://www.googleapis.com/auth/taskqueue
    Taskqueue,
    /// Scope for https://www.googleapis.com/auth/taskqueue.consumer
    TaskqueueConsumer,
    /// Scope for https://www.googleapis.com/auth/tasks
    Tasks,
    /// Scope for https://www.googleapis.com/auth/tasks.readonly
    TasksReadonly,
    /// Scope for https://www.googleapis.com/auth/urlshortener
    Urlshortener,
    /// Scope for https://www.googleapis.com/auth/user.addresses.read
    UserAddressesRead,
    /// Scope for https://www.googleapis.com/auth/user.birthday.read
    UserBirthdayRead,
    /// Scope for https://www.googleapis.com/auth/user.emails.read
    UserEmailsRead,
    /// Scope for https://www.googleapis.com/auth/user.phonenumbers.read
    UserPhonenumbersRead,
    /// Scope for https://www.googleapis.com/auth/userinfo.email
    UserinfoEmail,
    /// Scope for https://www.googleapis.com/auth/userinfo.profile
    UserinfoProfile,
    /// Scope for https://www.googleapis.com/auth/userlocation.beacon.registry
    UserlocationBeaconRegistry,
    /// Scope for https://www.googleapis.com/auth/webmasters
    Webmasters,
    /// Scope for https://www.googleapis.com/auth/webmasters.readonly
    WebmastersReadonly,
    /// Scope for https://www.googleapis.com/auth/xapi.zoo
    XapiZoo,
    /// Scope for https://www.googleapis.com/auth/youtube
    Youtube,
    /// Scope for https://www.googleapis.com/auth/youtube.force-ssl
    YoutubeForceSsl,
    /// Scope for https://www.googleapis.com/auth/youtube.readonly
    YoutubeReadonly,
    /// Scope for https://www.googleapis.com/auth/youtube.upload
    YoutubeUpload,
    /// Scope for https://www.googleapis.com/auth/youtubepartner
    Youtubepartner,
    /// Scope for https://www.googleapis.com/auth/youtubepartner-channel-audit
    YoutubepartnerChannelAudit,
    /// Scope for https://www.googleapis.com/auth/yt-analytics-monetary.readonly
    YtAnalyticsMonetaryReadonly,
    /// Scope for https://www.googleapis.com/auth/yt-analytics.readonly
    YtAnalyticsReadonly,
}
impl Scope {
    /// Return url assocaited with scope
    pub fn url(&self) -> &'static str {
        match *self {
             Scope::Activity => "https://www.googleapis.com/auth/activity",
             Scope::AdexchangeBuyer => "https://www.googleapis.com/auth/adexchange.buyer",
             Scope::AdexchangeSeller => "https://www.googleapis.com/auth/adexchange.seller",
             Scope::AdexchangeSellerReadonly => "https://www.googleapis.com/auth/adexchange.seller.readonly",
             Scope::AdminDatatransfer => "https://www.googleapis.com/auth/admin.datatransfer",
             Scope::AdminDatatransferReadonly => "https://www.googleapis.com/auth/admin.datatransfer.readonly",
             Scope::AdminDirectoryCustomer => "https://www.googleapis.com/auth/admin.directory.customer",
             Scope::AdminDirectoryCustomerReadonly => "https://www.googleapis.com/auth/admin.directory.customer.readonly",
             Scope::AdminDirectoryDeviceChromeos => "https://www.googleapis.com/auth/admin.directory.device.chromeos",
             Scope::AdminDirectoryDeviceChromeosReadonly => "https://www.googleapis.com/auth/admin.directory.device.chromeos.readonly",
             Scope::AdminDirectoryDeviceMobile => "https://www.googleapis.com/auth/admin.directory.device.mobile",
             Scope::AdminDirectoryDeviceMobileAction => "https://www.googleapis.com/auth/admin.directory.device.mobile.action",
             Scope::AdminDirectoryDeviceMobileReadonly => "https://www.googleapis.com/auth/admin.directory.device.mobile.readonly",
             Scope::AdminDirectoryDomain => "https://www.googleapis.com/auth/admin.directory.domain",
             Scope::AdminDirectoryDomainReadonly => "https://www.googleapis.com/auth/admin.directory.domain.readonly",
             Scope::AdminDirectoryGroup => "https://www.googleapis.com/auth/admin.directory.group",
             Scope::AdminDirectoryGroupMember => "https://www.googleapis.com/auth/admin.directory.group.member",
             Scope::AdminDirectoryGroupMemberReadonly => "https://www.googleapis.com/auth/admin.directory.group.member.readonly",
             Scope::AdminDirectoryGroupReadonly => "https://www.googleapis.com/auth/admin.directory.group.readonly",
             Scope::AdminDirectoryNotifications => "https://www.googleapis.com/auth/admin.directory.notifications",
             Scope::AdminDirectoryOrgunit => "https://www.googleapis.com/auth/admin.directory.orgunit",
             Scope::AdminDirectoryOrgunitReadonly => "https://www.googleapis.com/auth/admin.directory.orgunit.readonly",
             Scope::AdminDirectoryResourceCalendar => "https://www.googleapis.com/auth/admin.directory.resource.calendar",
             Scope::AdminDirectoryResourceCalendarReadonly => "https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly",
             Scope::AdminDirectoryRolemanagement => "https://www.googleapis.com/auth/admin.directory.rolemanagement",
             Scope::AdminDirectoryRolemanagementReadonly => "https://www.googleapis.com/auth/admin.directory.rolemanagement.readonly",
             Scope::AdminDirectoryUser => "https://www.googleapis.com/auth/admin.directory.user",
             Scope::AdminDirectoryUserAlias => "https://www.googleapis.com/auth/admin.directory.user.alias",
             Scope::AdminDirectoryUserAliasReadonly => "https://www.googleapis.com/auth/admin.directory.user.alias.readonly",
             Scope::AdminDirectoryUserReadonly => "https://www.googleapis.com/auth/admin.directory.user.readonly",
             Scope::AdminDirectoryUserSecurity => "https://www.googleapis.com/auth/admin.directory.user.security",
             Scope::AdminDirectoryUserschema => "https://www.googleapis.com/auth/admin.directory.userschema",
             Scope::AdminDirectoryUserschemaReadonly => "https://www.googleapis.com/auth/admin.directory.userschema.readonly",
             Scope::AdminReportsAuditReadonly => "https://www.googleapis.com/auth/admin.reports.audit.readonly",
             Scope::AdminReportsUsageReadonly => "https://www.googleapis.com/auth/admin.reports.usage.readonly",
             Scope::Adsense => "https://www.googleapis.com/auth/adsense",
             Scope::AdsenseReadonly => "https://www.googleapis.com/auth/adsense.readonly",
             Scope::Adsensehost => "https://www.googleapis.com/auth/adsensehost",
             Scope::Analytics => "https://www.googleapis.com/auth/analytics",
             Scope::AnalyticsEdit => "https://www.googleapis.com/auth/analytics.edit",
             Scope::AnalyticsManageUsers => "https://www.googleapis.com/auth/analytics.manage.users",
             Scope::AnalyticsManageUsersReadonly => "https://www.googleapis.com/auth/analytics.manage.users.readonly",
             Scope::AnalyticsProvision => "https://www.googleapis.com/auth/analytics.provision",
             Scope::AnalyticsReadonly => "https://www.googleapis.com/auth/analytics.readonly",
             Scope::Androidenterprise => "https://www.googleapis.com/auth/androidenterprise",
             Scope::Androidpublisher => "https://www.googleapis.com/auth/androidpublisher",
             Scope::AppengineAdmin => "https://www.googleapis.com/auth/appengine.admin",
             Scope::AppsGroupsMigration => "https://www.googleapis.com/auth/apps.groups.migration",
             Scope::AppsGroupsSettings => "https://www.googleapis.com/auth/apps.groups.settings",
             Scope::AppsLicensing => "https://www.googleapis.com/auth/apps.licensing",
             Scope::AppsOrder => "https://www.googleapis.com/auth/apps.order",
             Scope::AppsOrderReadonly => "https://www.googleapis.com/auth/apps.order.readonly",
             Scope::Appstate => "https://www.googleapis.com/auth/appstate",
             Scope::Bigquery => "https://www.googleapis.com/auth/bigquery",
             Scope::BigqueryInsertdata => "https://www.googleapis.com/auth/bigquery.insertdata",
             Scope::Blogger => "https://www.googleapis.com/auth/blogger",
             Scope::BloggerReadonly => "https://www.googleapis.com/auth/blogger.readonly",
             Scope::Books => "https://www.googleapis.com/auth/books",
             Scope::Calendar => "https://www.googleapis.com/auth/calendar",
             Scope::CalendarFeeds => "https://www.google.com/calendar/feeds",
             Scope::CalendarReadonly => "https://www.googleapis.com/auth/calendar.readonly",
             Scope::ClassroomCourses => "https://www.googleapis.com/auth/classroom.courses",
             Scope::ClassroomCoursesReadonly => "https://www.googleapis.com/auth/classroom.courses.readonly",
             Scope::ClassroomCourseworkMe => "https://www.googleapis.com/auth/classroom.coursework.me",
             Scope::ClassroomCourseworkMeReadonly => "https://www.googleapis.com/auth/classroom.coursework.me.readonly",
             Scope::ClassroomCourseworkStudents => "https://www.googleapis.com/auth/classroom.coursework.students",
             Scope::ClassroomCourseworkStudentsReadonly => "https://www.googleapis.com/auth/classroom.coursework.students.readonly",
             Scope::ClassroomGuardianlinksMeReadonly => "https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly",
             Scope::ClassroomGuardianlinksStudents => "https://www.googleapis.com/auth/classroom.guardianlinks.students",
             Scope::ClassroomGuardianlinksStudentsReadonly => "https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly",
             Scope::ClassroomProfileEmails => "https://www.googleapis.com/auth/classroom.profile.emails",
             Scope::ClassroomProfilePhotos => "https://www.googleapis.com/auth/classroom.profile.photos",
             Scope::ClassroomRosters => "https://www.googleapis.com/auth/classroom.rosters",
             Scope::ClassroomRostersReadonly => "https://www.googleapis.com/auth/classroom.rosters.readonly",
             Scope::ClassroomStudentSubmissionsMeReadonly => "https://www.googleapis.com/auth/classroom.student-submissions.me.readonly",
             Scope::ClassroomStudentSubmissionsStudentsReadonly => "https://www.googleapis.com/auth/classroom.student-submissions.students.readonly",
             Scope::CloudDebugger => "https://www.googleapis.com/auth/cloud_debugger",
             Scope::CloudLanguage => "https://www.googleapis.com/auth/cloud-language",
             Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
             Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
             Scope::CloudTranslation => "https://www.googleapis.com/auth/cloud-translation",
             Scope::CloudUseraccounts => "https://www.googleapis.com/auth/cloud.useraccounts",
             Scope::CloudUseraccountsReadonly => "https://www.googleapis.com/auth/cloud.useraccounts.readonly",
             Scope::Cloudruntimeconfig => "https://www.googleapis.com/auth/cloudruntimeconfig",
             Scope::Compute => "https://www.googleapis.com/auth/compute",
             Scope::ComputeReadonly => "https://www.googleapis.com/auth/compute.readonly",
             Scope::Contacts => "https://www.googleapis.com/auth/contacts",
             Scope::ContactsReadonly => "https://www.googleapis.com/auth/contacts.readonly",
             Scope::Content => "https://www.googleapis.com/auth/content",
             Scope::Datastore => "https://www.googleapis.com/auth/datastore",
             Scope::Ddmconversions => "https://www.googleapis.com/auth/ddmconversions",
             Scope::DevstorageFullControl => "https://www.googleapis.com/auth/devstorage.full_control",
             Scope::DevstorageReadOnly => "https://www.googleapis.com/auth/devstorage.read_only",
             Scope::DevstorageReadWrite => "https://www.googleapis.com/auth/devstorage.read_write",
             Scope::Dfareporting => "https://www.googleapis.com/auth/dfareporting",
             Scope::Dfatrafficking => "https://www.googleapis.com/auth/dfatrafficking",
             Scope::Doubleclickbidmanager => "https://www.googleapis.com/auth/doubleclickbidmanager",
             Scope::Doubleclicksearch => "https://www.googleapis.com/auth/doubleclicksearch",
             Scope::Drive => "https://www.googleapis.com/auth/drive",
             Scope::DriveAppdata => "https://www.googleapis.com/auth/drive.appdata",
             Scope::DriveFile => "https://www.googleapis.com/auth/drive.file",
             Scope::DriveMetadata => "https://www.googleapis.com/auth/drive.metadata",
             Scope::DriveMetadataReadonly => "https://www.googleapis.com/auth/drive.metadata.readonly",
             Scope::DrivePhotosReadonly => "https://www.googleapis.com/auth/drive.photos.readonly",
             Scope::DriveReadonly => "https://www.googleapis.com/auth/drive.readonly",
             Scope::DriveScripts => "https://www.googleapis.com/auth/drive.scripts",
             Scope::Firebase => "https://www.googleapis.com/auth/firebase",
             Scope::FirebaseReadonly => "https://www.googleapis.com/auth/firebase.readonly",
             Scope::FitnessActivityRead => "https://www.googleapis.com/auth/fitness.activity.read",
             Scope::FitnessActivityWrite => "https://www.googleapis.com/auth/fitness.activity.write",
             Scope::FitnessBloodGlucoseRead => "https://www.googleapis.com/auth/fitness.blood_glucose.read",
             Scope::FitnessBloodGlucoseWrite => "https://www.googleapis.com/auth/fitness.blood_glucose.write",
             Scope::FitnessBloodPressureRead => "https://www.googleapis.com/auth/fitness.blood_pressure.read",
             Scope::FitnessBloodPressureWrite => "https://www.googleapis.com/auth/fitness.blood_pressure.write",
             Scope::FitnessBodyRead => "https://www.googleapis.com/auth/fitness.body.read",
             Scope::FitnessBodyTemperatureRead => "https://www.googleapis.com/auth/fitness.body_temperature.read",
             Scope::FitnessBodyTemperatureWrite => "https://www.googleapis.com/auth/fitness.body_temperature.write",
             Scope::FitnessBodyWrite => "https://www.googleapis.com/auth/fitness.body.write",
             Scope::FitnessLocationRead => "https://www.googleapis.com/auth/fitness.location.read",
             Scope::FitnessLocationWrite => "https://www.googleapis.com/auth/fitness.location.write",
             Scope::FitnessNutritionRead => "https://www.googleapis.com/auth/fitness.nutrition.read",
             Scope::FitnessNutritionWrite => "https://www.googleapis.com/auth/fitness.nutrition.write",
             Scope::FitnessOxygenSaturationRead => "https://www.googleapis.com/auth/fitness.oxygen_saturation.read",
             Scope::FitnessOxygenSaturationWrite => "https://www.googleapis.com/auth/fitness.oxygen_saturation.write",
             Scope::FitnessReproductiveHealthRead => "https://www.googleapis.com/auth/fitness.reproductive_health.read",
             Scope::FitnessReproductiveHealthWrite => "https://www.googleapis.com/auth/fitness.reproductive_health.write",
             Scope::Forms => "https://www.googleapis.com/auth/forms",
             Scope::FormsCurrentonly => "https://www.googleapis.com/auth/forms.currentonly",
             Scope::Fusiontables => "https://www.googleapis.com/auth/fusiontables",
             Scope::FusiontablesReadonly => "https://www.googleapis.com/auth/fusiontables.readonly",
             Scope::Games => "https://www.googleapis.com/auth/games",
             Scope::Genomics => "https://www.googleapis.com/auth/genomics",
             Scope::GenomicsReadonly => "https://www.googleapis.com/auth/genomics.readonly",
             Scope::GlassLocation => "https://www.googleapis.com/auth/glass.location",
             Scope::GlassTimeline => "https://www.googleapis.com/auth/glass.timeline",
             Scope::Gmail => "https://mail.google.com/",
             Scope::GmailCompose => "https://www.googleapis.com/auth/gmail.compose",
             Scope::GmailInsert => "https://www.googleapis.com/auth/gmail.insert",
             Scope::GmailLabels => "https://www.googleapis.com/auth/gmail.labels",
             Scope::GmailMetadata => "https://www.googleapis.com/auth/gmail.metadata",
             Scope::GmailModify => "https://www.googleapis.com/auth/gmail.modify",
             Scope::GmailReadonly => "https://www.googleapis.com/auth/gmail.readonly",
             Scope::GmailSend => "https://www.googleapis.com/auth/gmail.send",
             Scope::GmailSettingsBasic => "https://www.googleapis.com/auth/gmail.settings.basic",
             Scope::GmailSettingsSharing => "https://www.googleapis.com/auth/gmail.settings.sharing",
             Scope::Groups => "https://www.googleapis.com/auth/groups",
             Scope::LoggingAdmin => "https://www.googleapis.com/auth/logging.admin",
             Scope::LoggingRead => "https://www.googleapis.com/auth/logging.read",
             Scope::LoggingWrite => "https://www.googleapis.com/auth/logging.write",
             Scope::M8Feeds => "https://www.google.com/m8/feeds",
             Scope::Manufacturercenter => "https://www.googleapis.com/auth/manufacturercenter",
             Scope::Monitoring => "https://www.googleapis.com/auth/monitoring",
             Scope::MonitoringRead => "https://www.googleapis.com/auth/monitoring.read",
             Scope::MonitoringWrite => "https://www.googleapis.com/auth/monitoring.write",
             Scope::NdevClouddnsReadonly => "https://www.googleapis.com/auth/ndev.clouddns.readonly",
             Scope::NdevClouddnsReadwrite => "https://www.googleapis.com/auth/ndev.clouddns.readwrite",
             Scope::NdevCloudman => "https://www.googleapis.com/auth/ndev.cloudman",
             Scope::NdevCloudmanReadonly => "https://www.googleapis.com/auth/ndev.cloudman.readonly",
             Scope::PlaymoviesPartnerReadonly => "https://www.googleapis.com/auth/playmovies_partner.readonly",
             Scope::PlusCirclesRead => "https://www.googleapis.com/auth/plus.circles.read",
             Scope::PlusCirclesWrite => "https://www.googleapis.com/auth/plus.circles.write",
             Scope::PlusLogin => "https://www.googleapis.com/auth/plus.login",
             Scope::PlusMe => "https://www.googleapis.com/auth/plus.me",
             Scope::PlusMediaUpload => "https://www.googleapis.com/auth/plus.media.upload",
             Scope::PlusProfilesRead => "https://www.googleapis.com/auth/plus.profiles.read",
             Scope::PlusStreamRead => "https://www.googleapis.com/auth/plus.stream.read",
             Scope::PlusStreamWrite => "https://www.googleapis.com/auth/plus.stream.write",
             Scope::Prediction => "https://www.googleapis.com/auth/prediction",
             Scope::Presentations => "https://www.googleapis.com/auth/presentations",
             Scope::PresentationsReadonly => "https://www.googleapis.com/auth/presentations.readonly",
             Scope::Pubsub => "https://www.googleapis.com/auth/pubsub",
             Scope::Replicapool => "https://www.googleapis.com/auth/replicapool",
             Scope::ReplicapoolReadonly => "https://www.googleapis.com/auth/replicapool.readonly",
             Scope::ServiceManagement => "https://www.googleapis.com/auth/service.management",
             Scope::ServiceManagementReadonly => "https://www.googleapis.com/auth/service.management.readonly",
             Scope::Servicecontrol => "https://www.googleapis.com/auth/servicecontrol",
             Scope::Siteverification => "https://www.googleapis.com/auth/siteverification",
             Scope::SiteverificationVerifyOnly => "https://www.googleapis.com/auth/siteverification.verify_only",
             Scope::SourceReadOnly => "https://www.googleapis.com/auth/source.read_only",
             Scope::SourceReadWrite => "https://www.googleapis.com/auth/source.read_write",
             Scope::SpannerAdmin => "https://www.googleapis.com/auth/spanner.admin",
             Scope::SpannerData => "https://www.googleapis.com/auth/spanner.data",
             Scope::Spreadsheets => "https://www.googleapis.com/auth/spreadsheets",
             Scope::SpreadsheetsReadonly => "https://www.googleapis.com/auth/spreadsheets.readonly",
             Scope::SqlserviceAdmin => "https://www.googleapis.com/auth/sqlservice.admin",
             Scope::TagmanagerDeleteContainers => "https://www.googleapis.com/auth/tagmanager.delete.containers",
             Scope::TagmanagerEditContainers => "https://www.googleapis.com/auth/tagmanager.edit.containers",
             Scope::TagmanagerEditContainerversions => "https://www.googleapis.com/auth/tagmanager.edit.containerversions",
             Scope::TagmanagerManageAccounts => "https://www.googleapis.com/auth/tagmanager.manage.accounts",
             Scope::TagmanagerManageUsers => "https://www.googleapis.com/auth/tagmanager.manage.users",
             Scope::TagmanagerPublish => "https://www.googleapis.com/auth/tagmanager.publish",
             Scope::TagmanagerReadonly => "https://www.googleapis.com/auth/tagmanager.readonly",
             Scope::Taskqueue => "https://www.googleapis.com/auth/taskqueue",
             Scope::TaskqueueConsumer => "https://www.googleapis.com/auth/taskqueue.consumer",
             Scope::Tasks => "https://www.googleapis.com/auth/tasks",
             Scope::TasksReadonly => "https://www.googleapis.com/auth/tasks.readonly",
             Scope::Urlshortener => "https://www.googleapis.com/auth/urlshortener",
             Scope::UserAddressesRead => "https://www.googleapis.com/auth/user.addresses.read",
             Scope::UserBirthdayRead => "https://www.googleapis.com/auth/user.birthday.read",
             Scope::UserEmailsRead => "https://www.googleapis.com/auth/user.emails.read",
             Scope::UserPhonenumbersRead => "https://www.googleapis.com/auth/user.phonenumbers.read",
             Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
             Scope::UserinfoProfile => "https://www.googleapis.com/auth/userinfo.profile",
             Scope::UserlocationBeaconRegistry => "https://www.googleapis.com/auth/userlocation.beacon.registry",
             Scope::Webmasters => "https://www.googleapis.com/auth/webmasters",
             Scope::WebmastersReadonly => "https://www.googleapis.com/auth/webmasters.readonly",
             Scope::XapiZoo => "https://www.googleapis.com/auth/xapi.zoo",
             Scope::Youtube => "https://www.googleapis.com/auth/youtube",
             Scope::YoutubeForceSsl => "https://www.googleapis.com/auth/youtube.force-ssl",
             Scope::YoutubeReadonly => "https://www.googleapis.com/auth/youtube.readonly",
             Scope::YoutubeUpload => "https://www.googleapis.com/auth/youtube.upload",
             Scope::Youtubepartner => "https://www.googleapis.com/auth/youtubepartner",
             Scope::YoutubepartnerChannelAudit => "https://www.googleapis.com/auth/youtubepartner-channel-audit",
             Scope::YtAnalyticsMonetaryReadonly => "https://www.googleapis.com/auth/yt-analytics-monetary.readonly",
             Scope::YtAnalyticsReadonly => "https://www.googleapis.com/auth/yt-analytics.readonly",
        }
    }
    /// Return Scope assocaited with url
    pub fn scope(url: &str) -> Option<Scope> {
        match url {
             "https://www.googleapis.com/auth/activity" => Some(Scope::Activity),
             "https://www.googleapis.com/auth/adexchange.buyer" => Some(Scope::AdexchangeBuyer),
             "https://www.googleapis.com/auth/adexchange.seller" => Some(Scope::AdexchangeSeller),
             "https://www.googleapis.com/auth/adexchange.seller.readonly" => Some(Scope::AdexchangeSellerReadonly),
             "https://www.googleapis.com/auth/admin.datatransfer" => Some(Scope::AdminDatatransfer),
             "https://www.googleapis.com/auth/admin.datatransfer.readonly" => Some(Scope::AdminDatatransferReadonly),
             "https://www.googleapis.com/auth/admin.directory.customer" => Some(Scope::AdminDirectoryCustomer),
             "https://www.googleapis.com/auth/admin.directory.customer.readonly" => Some(Scope::AdminDirectoryCustomerReadonly),
             "https://www.googleapis.com/auth/admin.directory.device.chromeos" => Some(Scope::AdminDirectoryDeviceChromeos),
             "https://www.googleapis.com/auth/admin.directory.device.chromeos.readonly" => Some(Scope::AdminDirectoryDeviceChromeosReadonly),
             "https://www.googleapis.com/auth/admin.directory.device.mobile" => Some(Scope::AdminDirectoryDeviceMobile),
             "https://www.googleapis.com/auth/admin.directory.device.mobile.action" => Some(Scope::AdminDirectoryDeviceMobileAction),
             "https://www.googleapis.com/auth/admin.directory.device.mobile.readonly" => Some(Scope::AdminDirectoryDeviceMobileReadonly),
             "https://www.googleapis.com/auth/admin.directory.domain" => Some(Scope::AdminDirectoryDomain),
             "https://www.googleapis.com/auth/admin.directory.domain.readonly" => Some(Scope::AdminDirectoryDomainReadonly),
             "https://www.googleapis.com/auth/admin.directory.group" => Some(Scope::AdminDirectoryGroup),
             "https://www.googleapis.com/auth/admin.directory.group.member" => Some(Scope::AdminDirectoryGroupMember),
             "https://www.googleapis.com/auth/admin.directory.group.member.readonly" => Some(Scope::AdminDirectoryGroupMemberReadonly),
             "https://www.googleapis.com/auth/admin.directory.group.readonly" => Some(Scope::AdminDirectoryGroupReadonly),
             "https://www.googleapis.com/auth/admin.directory.notifications" => Some(Scope::AdminDirectoryNotifications),
             "https://www.googleapis.com/auth/admin.directory.orgunit" => Some(Scope::AdminDirectoryOrgunit),
             "https://www.googleapis.com/auth/admin.directory.orgunit.readonly" => Some(Scope::AdminDirectoryOrgunitReadonly),
             "https://www.googleapis.com/auth/admin.directory.resource.calendar" => Some(Scope::AdminDirectoryResourceCalendar),
             "https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly" => Some(Scope::AdminDirectoryResourceCalendarReadonly),
             "https://www.googleapis.com/auth/admin.directory.rolemanagement" => Some(Scope::AdminDirectoryRolemanagement),
             "https://www.googleapis.com/auth/admin.directory.rolemanagement.readonly" => Some(Scope::AdminDirectoryRolemanagementReadonly),
             "https://www.googleapis.com/auth/admin.directory.user" => Some(Scope::AdminDirectoryUser),
             "https://www.googleapis.com/auth/admin.directory.user.alias" => Some(Scope::AdminDirectoryUserAlias),
             "https://www.googleapis.com/auth/admin.directory.user.alias.readonly" => Some(Scope::AdminDirectoryUserAliasReadonly),
             "https://www.googleapis.com/auth/admin.directory.user.readonly" => Some(Scope::AdminDirectoryUserReadonly),
             "https://www.googleapis.com/auth/admin.directory.user.security" => Some(Scope::AdminDirectoryUserSecurity),
             "https://www.googleapis.com/auth/admin.directory.userschema" => Some(Scope::AdminDirectoryUserschema),
             "https://www.googleapis.com/auth/admin.directory.userschema.readonly" => Some(Scope::AdminDirectoryUserschemaReadonly),
             "https://www.googleapis.com/auth/admin.reports.audit.readonly" => Some(Scope::AdminReportsAuditReadonly),
             "https://www.googleapis.com/auth/admin.reports.usage.readonly" => Some(Scope::AdminReportsUsageReadonly),
             "https://www.googleapis.com/auth/adsense" => Some(Scope::Adsense),
             "https://www.googleapis.com/auth/adsense.readonly" => Some(Scope::AdsenseReadonly),
             "https://www.googleapis.com/auth/adsensehost" => Some(Scope::Adsensehost),
             "https://www.googleapis.com/auth/analytics" => Some(Scope::Analytics),
             "https://www.googleapis.com/auth/analytics.edit" => Some(Scope::AnalyticsEdit),
             "https://www.googleapis.com/auth/analytics.manage.users" => Some(Scope::AnalyticsManageUsers),
             "https://www.googleapis.com/auth/analytics.manage.users.readonly" => Some(Scope::AnalyticsManageUsersReadonly),
             "https://www.googleapis.com/auth/analytics.provision" => Some(Scope::AnalyticsProvision),
             "https://www.googleapis.com/auth/analytics.readonly" => Some(Scope::AnalyticsReadonly),
             "https://www.googleapis.com/auth/androidenterprise" => Some(Scope::Androidenterprise),
             "https://www.googleapis.com/auth/androidpublisher" => Some(Scope::Androidpublisher),
             "https://www.googleapis.com/auth/appengine.admin" => Some(Scope::AppengineAdmin),
             "https://www.googleapis.com/auth/apps.groups.migration" => Some(Scope::AppsGroupsMigration),
             "https://www.googleapis.com/auth/apps.groups.settings" => Some(Scope::AppsGroupsSettings),
             "https://www.googleapis.com/auth/apps.licensing" => Some(Scope::AppsLicensing),
             "https://www.googleapis.com/auth/apps.order" => Some(Scope::AppsOrder),
             "https://www.googleapis.com/auth/apps.order.readonly" => Some(Scope::AppsOrderReadonly),
             "https://www.googleapis.com/auth/appstate" => Some(Scope::Appstate),
             "https://www.googleapis.com/auth/bigquery" => Some(Scope::Bigquery),
             "https://www.googleapis.com/auth/bigquery.insertdata" => Some(Scope::BigqueryInsertdata),
             "https://www.googleapis.com/auth/blogger" => Some(Scope::Blogger),
             "https://www.googleapis.com/auth/blogger.readonly" => Some(Scope::BloggerReadonly),
             "https://www.googleapis.com/auth/books" => Some(Scope::Books),
             "https://www.googleapis.com/auth/calendar" => Some(Scope::Calendar),
             "https://www.google.com/calendar/feeds" => Some(Scope::CalendarFeeds),
             "https://www.googleapis.com/auth/calendar.readonly" => Some(Scope::CalendarReadonly),
             "https://www.googleapis.com/auth/classroom.courses" => Some(Scope::ClassroomCourses),
             "https://www.googleapis.com/auth/classroom.courses.readonly" => Some(Scope::ClassroomCoursesReadonly),
             "https://www.googleapis.com/auth/classroom.coursework.me" => Some(Scope::ClassroomCourseworkMe),
             "https://www.googleapis.com/auth/classroom.coursework.me.readonly" => Some(Scope::ClassroomCourseworkMeReadonly),
             "https://www.googleapis.com/auth/classroom.coursework.students" => Some(Scope::ClassroomCourseworkStudents),
             "https://www.googleapis.com/auth/classroom.coursework.students.readonly" => Some(Scope::ClassroomCourseworkStudentsReadonly),
             "https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly" => Some(Scope::ClassroomGuardianlinksMeReadonly),
             "https://www.googleapis.com/auth/classroom.guardianlinks.students" => Some(Scope::ClassroomGuardianlinksStudents),
             "https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly" => Some(Scope::ClassroomGuardianlinksStudentsReadonly),
             "https://www.googleapis.com/auth/classroom.profile.emails" => Some(Scope::ClassroomProfileEmails),
             "https://www.googleapis.com/auth/classroom.profile.photos" => Some(Scope::ClassroomProfilePhotos),
             "https://www.googleapis.com/auth/classroom.rosters" => Some(Scope::ClassroomRosters),
             "https://www.googleapis.com/auth/classroom.rosters.readonly" => Some(Scope::ClassroomRostersReadonly),
             "https://www.googleapis.com/auth/classroom.student-submissions.me.readonly" => Some(Scope::ClassroomStudentSubmissionsMeReadonly),
             "https://www.googleapis.com/auth/classroom.student-submissions.students.readonly" => Some(Scope::ClassroomStudentSubmissionsStudentsReadonly),
             "https://www.googleapis.com/auth/cloud_debugger" => Some(Scope::CloudDebugger),
             "https://www.googleapis.com/auth/cloud-language" => Some(Scope::CloudLanguage),
             "https://www.googleapis.com/auth/cloud-platform" => Some(Scope::CloudPlatform),
             "https://www.googleapis.com/auth/cloud-platform.read-only" => Some(Scope::CloudPlatformReadOnly),
             "https://www.googleapis.com/auth/cloud-translation" => Some(Scope::CloudTranslation),
             "https://www.googleapis.com/auth/cloud.useraccounts" => Some(Scope::CloudUseraccounts),
             "https://www.googleapis.com/auth/cloud.useraccounts.readonly" => Some(Scope::CloudUseraccountsReadonly),
             "https://www.googleapis.com/auth/cloudruntimeconfig" => Some(Scope::Cloudruntimeconfig),
             "https://www.googleapis.com/auth/compute" => Some(Scope::Compute),
             "https://www.googleapis.com/auth/compute.readonly" => Some(Scope::ComputeReadonly),
             "https://www.googleapis.com/auth/contacts" => Some(Scope::Contacts),
             "https://www.googleapis.com/auth/contacts.readonly" => Some(Scope::ContactsReadonly),
             "https://www.googleapis.com/auth/content" => Some(Scope::Content),
             "https://www.googleapis.com/auth/datastore" => Some(Scope::Datastore),
             "https://www.googleapis.com/auth/ddmconversions" => Some(Scope::Ddmconversions),
             "https://www.googleapis.com/auth/devstorage.full_control" => Some(Scope::DevstorageFullControl),
             "https://www.googleapis.com/auth/devstorage.read_only" => Some(Scope::DevstorageReadOnly),
             "https://www.googleapis.com/auth/devstorage.read_write" => Some(Scope::DevstorageReadWrite),
             "https://www.googleapis.com/auth/dfareporting" => Some(Scope::Dfareporting),
             "https://www.googleapis.com/auth/dfatrafficking" => Some(Scope::Dfatrafficking),
             "https://www.googleapis.com/auth/doubleclickbidmanager" => Some(Scope::Doubleclickbidmanager),
             "https://www.googleapis.com/auth/doubleclicksearch" => Some(Scope::Doubleclicksearch),
             "https://www.googleapis.com/auth/drive" => Some(Scope::Drive),
             "https://www.googleapis.com/auth/drive.appdata" => Some(Scope::DriveAppdata),
             "https://www.googleapis.com/auth/drive.file" => Some(Scope::DriveFile),
             "https://www.googleapis.com/auth/drive.metadata" => Some(Scope::DriveMetadata),
             "https://www.googleapis.com/auth/drive.metadata.readonly" => Some(Scope::DriveMetadataReadonly),
             "https://www.googleapis.com/auth/drive.photos.readonly" => Some(Scope::DrivePhotosReadonly),
             "https://www.googleapis.com/auth/drive.readonly" => Some(Scope::DriveReadonly),
             "https://www.googleapis.com/auth/drive.scripts" => Some(Scope::DriveScripts),
             "https://www.googleapis.com/auth/firebase" => Some(Scope::Firebase),
             "https://www.googleapis.com/auth/firebase.readonly" => Some(Scope::FirebaseReadonly),
             "https://www.googleapis.com/auth/fitness.activity.read" => Some(Scope::FitnessActivityRead),
             "https://www.googleapis.com/auth/fitness.activity.write" => Some(Scope::FitnessActivityWrite),
             "https://www.googleapis.com/auth/fitness.blood_glucose.read" => Some(Scope::FitnessBloodGlucoseRead),
             "https://www.googleapis.com/auth/fitness.blood_glucose.write" => Some(Scope::FitnessBloodGlucoseWrite),
             "https://www.googleapis.com/auth/fitness.blood_pressure.read" => Some(Scope::FitnessBloodPressureRead),
             "https://www.googleapis.com/auth/fitness.blood_pressure.write" => Some(Scope::FitnessBloodPressureWrite),
             "https://www.googleapis.com/auth/fitness.body.read" => Some(Scope::FitnessBodyRead),
             "https://www.googleapis.com/auth/fitness.body_temperature.read" => Some(Scope::FitnessBodyTemperatureRead),
             "https://www.googleapis.com/auth/fitness.body_temperature.write" => Some(Scope::FitnessBodyTemperatureWrite),
             "https://www.googleapis.com/auth/fitness.body.write" => Some(Scope::FitnessBodyWrite),
             "https://www.googleapis.com/auth/fitness.location.read" => Some(Scope::FitnessLocationRead),
             "https://www.googleapis.com/auth/fitness.location.write" => Some(Scope::FitnessLocationWrite),
             "https://www.googleapis.com/auth/fitness.nutrition.read" => Some(Scope::FitnessNutritionRead),
             "https://www.googleapis.com/auth/fitness.nutrition.write" => Some(Scope::FitnessNutritionWrite),
             "https://www.googleapis.com/auth/fitness.oxygen_saturation.read" => Some(Scope::FitnessOxygenSaturationRead),
             "https://www.googleapis.com/auth/fitness.oxygen_saturation.write" => Some(Scope::FitnessOxygenSaturationWrite),
             "https://www.googleapis.com/auth/fitness.reproductive_health.read" => Some(Scope::FitnessReproductiveHealthRead),
             "https://www.googleapis.com/auth/fitness.reproductive_health.write" => Some(Scope::FitnessReproductiveHealthWrite),
             "https://www.googleapis.com/auth/forms" => Some(Scope::Forms),
             "https://www.googleapis.com/auth/forms.currentonly" => Some(Scope::FormsCurrentonly),
             "https://www.googleapis.com/auth/fusiontables" => Some(Scope::Fusiontables),
             "https://www.googleapis.com/auth/fusiontables.readonly" => Some(Scope::FusiontablesReadonly),
             "https://www.googleapis.com/auth/games" => Some(Scope::Games),
             "https://www.googleapis.com/auth/genomics" => Some(Scope::Genomics),
             "https://www.googleapis.com/auth/genomics.readonly" => Some(Scope::GenomicsReadonly),
             "https://www.googleapis.com/auth/glass.location" => Some(Scope::GlassLocation),
             "https://www.googleapis.com/auth/glass.timeline" => Some(Scope::GlassTimeline),
             "https://mail.google.com/" => Some(Scope::Gmail),
             "https://www.googleapis.com/auth/gmail.compose" => Some(Scope::GmailCompose),
             "https://www.googleapis.com/auth/gmail.insert" => Some(Scope::GmailInsert),
             "https://www.googleapis.com/auth/gmail.labels" => Some(Scope::GmailLabels),
             "https://www.googleapis.com/auth/gmail.metadata" => Some(Scope::GmailMetadata),
             "https://www.googleapis.com/auth/gmail.modify" => Some(Scope::GmailModify),
             "https://www.googleapis.com/auth/gmail.readonly" => Some(Scope::GmailReadonly),
             "https://www.googleapis.com/auth/gmail.send" => Some(Scope::GmailSend),
             "https://www.googleapis.com/auth/gmail.settings.basic" => Some(Scope::GmailSettingsBasic),
             "https://www.googleapis.com/auth/gmail.settings.sharing" => Some(Scope::GmailSettingsSharing),
             "https://www.googleapis.com/auth/groups" => Some(Scope::Groups),
             "https://www.googleapis.com/auth/logging.admin" => Some(Scope::LoggingAdmin),
             "https://www.googleapis.com/auth/logging.read" => Some(Scope::LoggingRead),
             "https://www.googleapis.com/auth/logging.write" => Some(Scope::LoggingWrite),
             "https://www.google.com/m8/feeds" => Some(Scope::M8Feeds),
             "https://www.googleapis.com/auth/manufacturercenter" => Some(Scope::Manufacturercenter),
             "https://www.googleapis.com/auth/monitoring" => Some(Scope::Monitoring),
             "https://www.googleapis.com/auth/monitoring.read" => Some(Scope::MonitoringRead),
             "https://www.googleapis.com/auth/monitoring.write" => Some(Scope::MonitoringWrite),
             "https://www.googleapis.com/auth/ndev.clouddns.readonly" => Some(Scope::NdevClouddnsReadonly),
             "https://www.googleapis.com/auth/ndev.clouddns.readwrite" => Some(Scope::NdevClouddnsReadwrite),
             "https://www.googleapis.com/auth/ndev.cloudman" => Some(Scope::NdevCloudman),
             "https://www.googleapis.com/auth/ndev.cloudman.readonly" => Some(Scope::NdevCloudmanReadonly),
             "https://www.googleapis.com/auth/playmovies_partner.readonly" => Some(Scope::PlaymoviesPartnerReadonly),
             "https://www.googleapis.com/auth/plus.circles.read" => Some(Scope::PlusCirclesRead),
             "https://www.googleapis.com/auth/plus.circles.write" => Some(Scope::PlusCirclesWrite),
             "https://www.googleapis.com/auth/plus.login" => Some(Scope::PlusLogin),
             "https://www.googleapis.com/auth/plus.me" => Some(Scope::PlusMe),
             "https://www.googleapis.com/auth/plus.media.upload" => Some(Scope::PlusMediaUpload),
             "https://www.googleapis.com/auth/plus.profiles.read" => Some(Scope::PlusProfilesRead),
             "https://www.googleapis.com/auth/plus.stream.read" => Some(Scope::PlusStreamRead),
             "https://www.googleapis.com/auth/plus.stream.write" => Some(Scope::PlusStreamWrite),
             "https://www.googleapis.com/auth/prediction" => Some(Scope::Prediction),
             "https://www.googleapis.com/auth/presentations" => Some(Scope::Presentations),
             "https://www.googleapis.com/auth/presentations.readonly" => Some(Scope::PresentationsReadonly),
             "https://www.googleapis.com/auth/pubsub" => Some(Scope::Pubsub),
             "https://www.googleapis.com/auth/replicapool" => Some(Scope::Replicapool),
             "https://www.googleapis.com/auth/replicapool.readonly" => Some(Scope::ReplicapoolReadonly),
             "https://www.googleapis.com/auth/service.management" => Some(Scope::ServiceManagement),
             "https://www.googleapis.com/auth/service.management.readonly" => Some(Scope::ServiceManagementReadonly),
             "https://www.googleapis.com/auth/servicecontrol" => Some(Scope::Servicecontrol),
             "https://www.googleapis.com/auth/siteverification" => Some(Scope::Siteverification),
             "https://www.googleapis.com/auth/siteverification.verify_only" => Some(Scope::SiteverificationVerifyOnly),
             "https://www.googleapis.com/auth/source.read_only" => Some(Scope::SourceReadOnly),
             "https://www.googleapis.com/auth/source.read_write" => Some(Scope::SourceReadWrite),
             "https://www.googleapis.com/auth/spanner.admin" => Some(Scope::SpannerAdmin),
             "https://www.googleapis.com/auth/spanner.data" => Some(Scope::SpannerData),
             "https://www.googleapis.com/auth/spreadsheets" => Some(Scope::Spreadsheets),
             "https://www.googleapis.com/auth/spreadsheets.readonly" => Some(Scope::SpreadsheetsReadonly),
             "https://www.googleapis.com/auth/sqlservice.admin" => Some(Scope::SqlserviceAdmin),
             "https://www.googleapis.com/auth/tagmanager.delete.containers" => Some(Scope::TagmanagerDeleteContainers),
             "https://www.googleapis.com/auth/tagmanager.edit.containers" => Some(Scope::TagmanagerEditContainers),
             "https://www.googleapis.com/auth/tagmanager.edit.containerversions" => Some(Scope::TagmanagerEditContainerversions),
             "https://www.googleapis.com/auth/tagmanager.manage.accounts" => Some(Scope::TagmanagerManageAccounts),
             "https://www.googleapis.com/auth/tagmanager.manage.users" => Some(Scope::TagmanagerManageUsers),
             "https://www.googleapis.com/auth/tagmanager.publish" => Some(Scope::TagmanagerPublish),
             "https://www.googleapis.com/auth/tagmanager.readonly" => Some(Scope::TagmanagerReadonly),
             "https://www.googleapis.com/auth/taskqueue" => Some(Scope::Taskqueue),
             "https://www.googleapis.com/auth/taskqueue.consumer" => Some(Scope::TaskqueueConsumer),
             "https://www.googleapis.com/auth/tasks" => Some(Scope::Tasks),
             "https://www.googleapis.com/auth/tasks.readonly" => Some(Scope::TasksReadonly),
             "https://www.googleapis.com/auth/urlshortener" => Some(Scope::Urlshortener),
             "https://www.googleapis.com/auth/user.addresses.read" => Some(Scope::UserAddressesRead),
             "https://www.googleapis.com/auth/user.birthday.read" => Some(Scope::UserBirthdayRead),
             "https://www.googleapis.com/auth/user.emails.read" => Some(Scope::UserEmailsRead),
             "https://www.googleapis.com/auth/user.phonenumbers.read" => Some(Scope::UserPhonenumbersRead),
             "https://www.googleapis.com/auth/userinfo.email" => Some(Scope::UserinfoEmail),
             "https://www.googleapis.com/auth/userinfo.profile" => Some(Scope::UserinfoProfile),
             "https://www.googleapis.com/auth/userlocation.beacon.registry" => Some(Scope::UserlocationBeaconRegistry),
             "https://www.googleapis.com/auth/webmasters" => Some(Scope::Webmasters),
             "https://www.googleapis.com/auth/webmasters.readonly" => Some(Scope::WebmastersReadonly),
             "https://www.googleapis.com/auth/xapi.zoo" => Some(Scope::XapiZoo),
             "https://www.googleapis.com/auth/youtube" => Some(Scope::Youtube),
             "https://www.googleapis.com/auth/youtube.force-ssl" => Some(Scope::YoutubeForceSsl),
             "https://www.googleapis.com/auth/youtube.readonly" => Some(Scope::YoutubeReadonly),
             "https://www.googleapis.com/auth/youtube.upload" => Some(Scope::YoutubeUpload),
             "https://www.googleapis.com/auth/youtubepartner" => Some(Scope::Youtubepartner),
             "https://www.googleapis.com/auth/youtubepartner-channel-audit" => Some(Scope::YoutubepartnerChannelAudit),
             "https://www.googleapis.com/auth/yt-analytics-monetary.readonly" => Some(Scope::YtAnalyticsMonetaryReadonly),
             "https://www.googleapis.com/auth/yt-analytics.readonly" => Some(Scope::YtAnalyticsReadonly),
                 _ => None
        }
    }
}