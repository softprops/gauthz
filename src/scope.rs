

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
#[derive(Debug, PartialEq)]
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
    /// A custom API scope
    Custom(&'static str)
}
impl Scope {
    /// Return url assocaited with scope
    pub fn url(&self) -> &'static str {
        match *self {
            Scope::Activity => "https://www.googleapis.com/auth/activity",
            Scope::AdexchangeBuyer => {
                "https://www.googleapis.com/auth/adexchange.buyer"
            }
            Scope::AdexchangeSeller => {
                "https://www.googleapis.com/auth/adexchange.seller"
            }
            Scope::AdexchangeSellerReadonly => {
                "https://www.googleapis.com/auth/adexchange.seller.readonly"
            }
            Scope::AdminDatatransfer => {
                "https://www.googleapis.com/auth/admin.datatransfer"
            }
            Scope::AdminDatatransferReadonly => {
                "https://www.googleapis.com/auth/admin.datatransfer.readonly"
            }
            Scope::AdminDirectoryCustomer => {
                "https://www.googleapis.com/auth/admin.directory.customer"
            }
            Scope::AdminDirectoryCustomerReadonly => {
                "https://www.googleapis.com/auth/admin.directory.customer.readonly"
            }
            Scope::AdminDirectoryDeviceChromeos => {
                "https://www.googleapis.com/auth/admin.directory.device.chromeos"
            }
            Scope::AdminDirectoryDeviceChromeosReadonly => {
                "https://www.googleapis.com/auth/admin.directory.device.chromeos.readonly"
            }
            Scope::AdminDirectoryDeviceMobile => {
                "https://www.googleapis.com/auth/admin.directory.device.mobile"
            }
            Scope::AdminDirectoryDeviceMobileAction => {
                "https://www.googleapis.com/auth/admin.directory.device.mobile.action"
            }
            Scope::AdminDirectoryDeviceMobileReadonly => {
                "https://www.googleapis.com/auth/admin.directory.device.mobile.readonly"
            }
            Scope::AdminDirectoryDomain => {
                "https://www.googleapis.com/auth/admin.directory.domain"
            }
            Scope::AdminDirectoryDomainReadonly => {
                "https://www.googleapis.com/auth/admin.directory.domain.readonly"
            }
            Scope::AdminDirectoryGroup => {
                "https://www.googleapis.com/auth/admin.directory.group"
            }
            Scope::AdminDirectoryGroupMember => {
                "https://www.googleapis.com/auth/admin.directory.group.member"
            }
            Scope::AdminDirectoryGroupMemberReadonly => {
                "https://www.googleapis.com/auth/admin.directory.group.member.readonly"
            }
            Scope::AdminDirectoryGroupReadonly => {
                "https://www.googleapis.com/auth/admin.directory.group.readonly"
            }
            Scope::AdminDirectoryNotifications => {
                "https://www.googleapis.com/auth/admin.directory.notifications"
            }
            Scope::AdminDirectoryOrgunit => {
                "https://www.googleapis.com/auth/admin.directory.orgunit"
            }
            Scope::AdminDirectoryOrgunitReadonly => {
                "https://www.googleapis.com/auth/admin.directory.orgunit.readonly"
            }
            Scope::AdminDirectoryResourceCalendar => {
                "https://www.googleapis.com/auth/admin.directory.resource.calendar"
            }
            Scope::AdminDirectoryResourceCalendarReadonly => {
                "https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly"
            }
            Scope::AdminDirectoryRolemanagement => {
                "https://www.googleapis.com/auth/admin.directory.rolemanagement"
            }
            Scope::AdminDirectoryRolemanagementReadonly => {
                "https://www.googleapis.com/auth/admin.directory.rolemanagement.readonly"
            }
            Scope::AdminDirectoryUser => {
                "https://www.googleapis.com/auth/admin.directory.user"
            }
            Scope::AdminDirectoryUserAlias => {
                "https://www.googleapis.com/auth/admin.directory.user.alias"
            }
            Scope::AdminDirectoryUserAliasReadonly => {
                "https://www.googleapis.com/auth/admin.directory.user.alias.readonly"
            }
            Scope::AdminDirectoryUserReadonly => {
                "https://www.googleapis.com/auth/admin.directory.user.readonly"
            }
            Scope::AdminDirectoryUserSecurity => {
                "https://www.googleapis.com/auth/admin.directory.user.security"
            }
            Scope::AdminDirectoryUserschema => {
                "https://www.googleapis.com/auth/admin.directory.userschema"
            }
            Scope::AdminDirectoryUserschemaReadonly => {
                "https://www.googleapis.com/auth/admin.directory.userschema.readonly"
            }
            Scope::AdminReportsAuditReadonly => {
                "https://www.googleapis.com/auth/admin.reports.audit.readonly"
            }
            Scope::AdminReportsUsageReadonly => {
                "https://www.googleapis.com/auth/admin.reports.usage.readonly"
            }
            Scope::Adsense => "https://www.googleapis.com/auth/adsense",
            Scope::AdsenseReadonly => {
                "https://www.googleapis.com/auth/adsense.readonly"
            }
            Scope::Adsensehost => "https://www.googleapis.com/auth/adsensehost",
            Scope::Analytics => "https://www.googleapis.com/auth/analytics",
            Scope::AnalyticsEdit => {
                "https://www.googleapis.com/auth/analytics.edit"
            }
            Scope::AnalyticsManageUsers => {
                "https://www.googleapis.com/auth/analytics.manage.users"
            }
            Scope::AnalyticsManageUsersReadonly => {
                "https://www.googleapis.com/auth/analytics.manage.users.readonly"
            }
            Scope::AnalyticsProvision => {
                "https://www.googleapis.com/auth/analytics.provision"
            }
            Scope::AnalyticsReadonly => {
                "https://www.googleapis.com/auth/analytics.readonly"
            }
            Scope::Androidenterprise => {
                "https://www.googleapis.com/auth/androidenterprise"
            }
            Scope::Androidpublisher => {
                "https://www.googleapis.com/auth/androidpublisher"
            }
            Scope::AppengineAdmin => {
                "https://www.googleapis.com/auth/appengine.admin"
            }
            Scope::AppsGroupsMigration => {
                "https://www.googleapis.com/auth/apps.groups.migration"
            }
            Scope::AppsGroupsSettings => {
                "https://www.googleapis.com/auth/apps.groups.settings"
            }
            Scope::AppsLicensing => {
                "https://www.googleapis.com/auth/apps.licensing"
            }
            Scope::AppsOrder => "https://www.googleapis.com/auth/apps.order",
            Scope::AppsOrderReadonly => {
                "https://www.googleapis.com/auth/apps.order.readonly"
            }
            Scope::Appstate => "https://www.googleapis.com/auth/appstate",
            Scope::Bigquery => "https://www.googleapis.com/auth/bigquery",
            Scope::BigqueryInsertdata => {
                "https://www.googleapis.com/auth/bigquery.insertdata"
            }
            Scope::Blogger => "https://www.googleapis.com/auth/blogger",
            Scope::BloggerReadonly => {
                "https://www.googleapis.com/auth/blogger.readonly"
            }
            Scope::Books => "https://www.googleapis.com/auth/books",
            Scope::Calendar => "https://www.googleapis.com/auth/calendar",
            Scope::CalendarFeeds => "https://www.google.com/calendar/feeds",
            Scope::CalendarReadonly => {
                "https://www.googleapis.com/auth/calendar.readonly"
            }
            Scope::ClassroomCourses => {
                "https://www.googleapis.com/auth/classroom.courses"
            }
            Scope::ClassroomCoursesReadonly => {
                "https://www.googleapis.com/auth/classroom.courses.readonly"
            }
            Scope::ClassroomCourseworkMe => {
                "https://www.googleapis.com/auth/classroom.coursework.me"
            }
            Scope::ClassroomCourseworkMeReadonly => {
                "https://www.googleapis.com/auth/classroom.coursework.me.readonly"
            }
            Scope::ClassroomCourseworkStudents => {
                "https://www.googleapis.com/auth/classroom.coursework.students"
            }
            Scope::ClassroomCourseworkStudentsReadonly => {
                "https://www.googleapis.com/auth/classroom.coursework.students.readonly"
            }
            Scope::ClassroomGuardianlinksMeReadonly => {
                "https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly"
            }
            Scope::ClassroomGuardianlinksStudents => {
                "https://www.googleapis.com/auth/classroom.guardianlinks.students"
            }
            Scope::ClassroomGuardianlinksStudentsReadonly => {
                "https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly"
            }
            Scope::ClassroomProfileEmails => {
                "https://www.googleapis.com/auth/classroom.profile.emails"
            }
            Scope::ClassroomProfilePhotos => {
                "https://www.googleapis.com/auth/classroom.profile.photos"
            }
            Scope::ClassroomRosters => {
                "https://www.googleapis.com/auth/classroom.rosters"
            }
            Scope::ClassroomRostersReadonly => {
                "https://www.googleapis.com/auth/classroom.rosters.readonly"
            }
            Scope::ClassroomStudentSubmissionsMeReadonly => {
                "https://www.googleapis.com/auth/classroom.student-submissions.me.readonly"
            }
            Scope::ClassroomStudentSubmissionsStudentsReadonly => {
                "https://www.googleapis.com/auth/classroom.student-submissions.students.readonly"
            }
            Scope::CloudDebugger => {
                "https://www.googleapis.com/auth/cloud_debugger"
            }
            Scope::CloudLanguage => {
                "https://www.googleapis.com/auth/cloud-language"
            }
            Scope::CloudPlatform => {
                "https://www.googleapis.com/auth/cloud-platform"
            }
            Scope::CloudPlatformReadOnly => {
                "https://www.googleapis.com/auth/cloud-platform.read-only"
            }
            Scope::CloudTranslation => {
                "https://www.googleapis.com/auth/cloud-translation"
            }
            Scope::CloudUseraccounts => {
                "https://www.googleapis.com/auth/cloud.useraccounts"
            }
            Scope::CloudUseraccountsReadonly => {
                "https://www.googleapis.com/auth/cloud.useraccounts.readonly"
            }
            Scope::Cloudruntimeconfig => {
                "https://www.googleapis.com/auth/cloudruntimeconfig"
            }
            Scope::Compute => "https://www.googleapis.com/auth/compute",
            Scope::ComputeReadonly => {
                "https://www.googleapis.com/auth/compute.readonly"
            }
            Scope::Contacts => "https://www.googleapis.com/auth/contacts",
            Scope::ContactsReadonly => {
                "https://www.googleapis.com/auth/contacts.readonly"
            }
            Scope::Content => "https://www.googleapis.com/auth/content",
            Scope::Datastore => "https://www.googleapis.com/auth/datastore",
            Scope::Ddmconversions => {
                "https://www.googleapis.com/auth/ddmconversions"
            }
            Scope::DevstorageFullControl => {
                "https://www.googleapis.com/auth/devstorage.full_control"
            }
            Scope::DevstorageReadOnly => {
                "https://www.googleapis.com/auth/devstorage.read_only"
            }
            Scope::DevstorageReadWrite => {
                "https://www.googleapis.com/auth/devstorage.read_write"
            }
            Scope::Dfareporting => {
                "https://www.googleapis.com/auth/dfareporting"
            }
            Scope::Dfatrafficking => {
                "https://www.googleapis.com/auth/dfatrafficking"
            }
            Scope::Doubleclickbidmanager => {
                "https://www.googleapis.com/auth/doubleclickbidmanager"
            }
            Scope::Doubleclicksearch => {
                "https://www.googleapis.com/auth/doubleclicksearch"
            }
            Scope::Drive => "https://www.googleapis.com/auth/drive",
            Scope::DriveAppdata => {
                "https://www.googleapis.com/auth/drive.appdata"
            }
            Scope::DriveFile => "https://www.googleapis.com/auth/drive.file",
            Scope::DriveMetadata => {
                "https://www.googleapis.com/auth/drive.metadata"
            }
            Scope::DriveMetadataReadonly => {
                "https://www.googleapis.com/auth/drive.metadata.readonly"
            }
            Scope::DrivePhotosReadonly => {
                "https://www.googleapis.com/auth/drive.photos.readonly"
            }
            Scope::DriveReadonly => {
                "https://www.googleapis.com/auth/drive.readonly"
            }
            Scope::DriveScripts => {
                "https://www.googleapis.com/auth/drive.scripts"
            }
            Scope::Firebase => "https://www.googleapis.com/auth/firebase",
            Scope::FirebaseReadonly => {
                "https://www.googleapis.com/auth/firebase.readonly"
            }
            Scope::FitnessActivityRead => {
                "https://www.googleapis.com/auth/fitness.activity.read"
            }
            Scope::FitnessActivityWrite => {
                "https://www.googleapis.com/auth/fitness.activity.write"
            }
            Scope::FitnessBloodGlucoseRead => {
                "https://www.googleapis.com/auth/fitness.blood_glucose.read"
            }
            Scope::FitnessBloodGlucoseWrite => {
                "https://www.googleapis.com/auth/fitness.blood_glucose.write"
            }
            Scope::FitnessBloodPressureRead => {
                "https://www.googleapis.com/auth/fitness.blood_pressure.read"
            }
            Scope::FitnessBloodPressureWrite => {
                "https://www.googleapis.com/auth/fitness.blood_pressure.write"
            }
            Scope::FitnessBodyRead => {
                "https://www.googleapis.com/auth/fitness.body.read"
            }
            Scope::FitnessBodyTemperatureRead => {
                "https://www.googleapis.com/auth/fitness.body_temperature.read"
            }
            Scope::FitnessBodyTemperatureWrite => {
                "https://www.googleapis.com/auth/fitness.body_temperature.write"
            }
            Scope::FitnessBodyWrite => {
                "https://www.googleapis.com/auth/fitness.body.write"
            }
            Scope::FitnessLocationRead => {
                "https://www.googleapis.com/auth/fitness.location.read"
            }
            Scope::FitnessLocationWrite => {
                "https://www.googleapis.com/auth/fitness.location.write"
            }
            Scope::FitnessNutritionRead => {
                "https://www.googleapis.com/auth/fitness.nutrition.read"
            }
            Scope::FitnessNutritionWrite => {
                "https://www.googleapis.com/auth/fitness.nutrition.write"
            }
            Scope::FitnessOxygenSaturationRead => {
                "https://www.googleapis.com/auth/fitness.oxygen_saturation.read"
            }
            Scope::FitnessOxygenSaturationWrite => {
                "https://www.googleapis.com/auth/fitness.oxygen_saturation.write"
            }
            Scope::FitnessReproductiveHealthRead => {
                "https://www.googleapis.com/auth/fitness.reproductive_health.read"
            }
            Scope::FitnessReproductiveHealthWrite => {
                "https://www.googleapis.com/auth/fitness.reproductive_health.write"
            }
            Scope::Forms => "https://www.googleapis.com/auth/forms",
            Scope::FormsCurrentonly => {
                "https://www.googleapis.com/auth/forms.currentonly"
            }
            Scope::Fusiontables => {
                "https://www.googleapis.com/auth/fusiontables"
            }
            Scope::FusiontablesReadonly => {
                "https://www.googleapis.com/auth/fusiontables.readonly"
            }
            Scope::Games => "https://www.googleapis.com/auth/games",
            Scope::Genomics => "https://www.googleapis.com/auth/genomics",
            Scope::GenomicsReadonly => {
                "https://www.googleapis.com/auth/genomics.readonly"
            }
            Scope::GlassLocation => {
                "https://www.googleapis.com/auth/glass.location"
            }
            Scope::GlassTimeline => {
                "https://www.googleapis.com/auth/glass.timeline"
            }
            Scope::Gmail => "https://mail.google.com/",
            Scope::GmailCompose => {
                "https://www.googleapis.com/auth/gmail.compose"
            }
            Scope::GmailInsert => {
                "https://www.googleapis.com/auth/gmail.insert"
            }
            Scope::GmailLabels => {
                "https://www.googleapis.com/auth/gmail.labels"
            }
            Scope::GmailMetadata => {
                "https://www.googleapis.com/auth/gmail.metadata"
            }
            Scope::GmailModify => {
                "https://www.googleapis.com/auth/gmail.modify"
            }
            Scope::GmailReadonly => {
                "https://www.googleapis.com/auth/gmail.readonly"
            }
            Scope::GmailSend => "https://www.googleapis.com/auth/gmail.send",
            Scope::GmailSettingsBasic => {
                "https://www.googleapis.com/auth/gmail.settings.basic"
            }
            Scope::GmailSettingsSharing => {
                "https://www.googleapis.com/auth/gmail.settings.sharing"
            }
            Scope::Groups => "https://www.googleapis.com/auth/groups",
            Scope::LoggingAdmin => {
                "https://www.googleapis.com/auth/logging.admin"
            }
            Scope::LoggingRead => {
                "https://www.googleapis.com/auth/logging.read"
            }
            Scope::LoggingWrite => {
                "https://www.googleapis.com/auth/logging.write"
            }
            Scope::M8Feeds => "https://www.google.com/m8/feeds",
            Scope::Manufacturercenter => {
                "https://www.googleapis.com/auth/manufacturercenter"
            }
            Scope::Monitoring => "https://www.googleapis.com/auth/monitoring",
            Scope::MonitoringRead => {
                "https://www.googleapis.com/auth/monitoring.read"
            }
            Scope::MonitoringWrite => {
                "https://www.googleapis.com/auth/monitoring.write"
            }
            Scope::NdevClouddnsReadonly => {
                "https://www.googleapis.com/auth/ndev.clouddns.readonly"
            }
            Scope::NdevClouddnsReadwrite => {
                "https://www.googleapis.com/auth/ndev.clouddns.readwrite"
            }
            Scope::NdevCloudman => {
                "https://www.googleapis.com/auth/ndev.cloudman"
            }
            Scope::NdevCloudmanReadonly => {
                "https://www.googleapis.com/auth/ndev.cloudman.readonly"
            }
            Scope::PlaymoviesPartnerReadonly => {
                "https://www.googleapis.com/auth/playmovies_partner.readonly"
            }
            Scope::PlusCirclesRead => {
                "https://www.googleapis.com/auth/plus.circles.read"
            }
            Scope::PlusCirclesWrite => {
                "https://www.googleapis.com/auth/plus.circles.write"
            }
            Scope::PlusLogin => "https://www.googleapis.com/auth/plus.login",
            Scope::PlusMe => "https://www.googleapis.com/auth/plus.me",
            Scope::PlusMediaUpload => {
                "https://www.googleapis.com/auth/plus.media.upload"
            }
            Scope::PlusProfilesRead => {
                "https://www.googleapis.com/auth/plus.profiles.read"
            }
            Scope::PlusStreamRead => {
                "https://www.googleapis.com/auth/plus.stream.read"
            }
            Scope::PlusStreamWrite => {
                "https://www.googleapis.com/auth/plus.stream.write"
            }
            Scope::Prediction => "https://www.googleapis.com/auth/prediction",
            Scope::Presentations => {
                "https://www.googleapis.com/auth/presentations"
            }
            Scope::PresentationsReadonly => {
                "https://www.googleapis.com/auth/presentations.readonly"
            }
            Scope::Pubsub => "https://www.googleapis.com/auth/pubsub",
            Scope::Replicapool => "https://www.googleapis.com/auth/replicapool",
            Scope::ReplicapoolReadonly => {
                "https://www.googleapis.com/auth/replicapool.readonly"
            }
            Scope::ServiceManagement => {
                "https://www.googleapis.com/auth/service.management"
            }
            Scope::ServiceManagementReadonly => {
                "https://www.googleapis.com/auth/service.management.readonly"
            }
            Scope::Servicecontrol => {
                "https://www.googleapis.com/auth/servicecontrol"
            }
            Scope::Siteverification => {
                "https://www.googleapis.com/auth/siteverification"
            }
            Scope::SiteverificationVerifyOnly => {
                "https://www.googleapis.com/auth/siteverification.verify_only"
            }
            Scope::SourceReadOnly => {
                "https://www.googleapis.com/auth/source.read_only"
            }
            Scope::SourceReadWrite => {
                "https://www.googleapis.com/auth/source.read_write"
            }
            Scope::SpannerAdmin => {
                "https://www.googleapis.com/auth/spanner.admin"
            }
            Scope::SpannerData => {
                "https://www.googleapis.com/auth/spanner.data"
            }
            Scope::Spreadsheets => {
                "https://www.googleapis.com/auth/spreadsheets"
            }
            Scope::SpreadsheetsReadonly => {
                "https://www.googleapis.com/auth/spreadsheets.readonly"
            }
            Scope::SqlserviceAdmin => {
                "https://www.googleapis.com/auth/sqlservice.admin"
            }
            Scope::TagmanagerDeleteContainers => {
                "https://www.googleapis.com/auth/tagmanager.delete.containers"
            }
            Scope::TagmanagerEditContainers => {
                "https://www.googleapis.com/auth/tagmanager.edit.containers"
            }
            Scope::TagmanagerEditContainerversions => {
                "https://www.googleapis.com/auth/tagmanager.edit.containerversions"
            }
            Scope::TagmanagerManageAccounts => {
                "https://www.googleapis.com/auth/tagmanager.manage.accounts"
            }
            Scope::TagmanagerManageUsers => {
                "https://www.googleapis.com/auth/tagmanager.manage.users"
            }
            Scope::TagmanagerPublish => {
                "https://www.googleapis.com/auth/tagmanager.publish"
            }
            Scope::TagmanagerReadonly => {
                "https://www.googleapis.com/auth/tagmanager.readonly"
            }
            Scope::Taskqueue => "https://www.googleapis.com/auth/taskqueue",
            Scope::TaskqueueConsumer => {
                "https://www.googleapis.com/auth/taskqueue.consumer"
            }
            Scope::Tasks => "https://www.googleapis.com/auth/tasks",
            Scope::TasksReadonly => {
                "https://www.googleapis.com/auth/tasks.readonly"
            }
            Scope::Urlshortener => {
                "https://www.googleapis.com/auth/urlshortener"
            }
            Scope::UserAddressesRead => {
                "https://www.googleapis.com/auth/user.addresses.read"
            }
            Scope::UserBirthdayRead => {
                "https://www.googleapis.com/auth/user.birthday.read"
            }
            Scope::UserEmailsRead => {
                "https://www.googleapis.com/auth/user.emails.read"
            }
            Scope::UserPhonenumbersRead => {
                "https://www.googleapis.com/auth/user.phonenumbers.read"
            }
            Scope::UserinfoEmail => {
                "https://www.googleapis.com/auth/userinfo.email"
            }
            Scope::UserinfoProfile => {
                "https://www.googleapis.com/auth/userinfo.profile"
            }
            Scope::UserlocationBeaconRegistry => {
                "https://www.googleapis.com/auth/userlocation.beacon.registry"
            }
            Scope::Webmasters => "https://www.googleapis.com/auth/webmasters",
            Scope::WebmastersReadonly => {
                "https://www.googleapis.com/auth/webmasters.readonly"
            }
            Scope::XapiZoo => "https://www.googleapis.com/auth/xapi.zoo",
            Scope::Youtube => "https://www.googleapis.com/auth/youtube",
            Scope::YoutubeForceSsl => {
                "https://www.googleapis.com/auth/youtube.force-ssl"
            }
            Scope::YoutubeReadonly => {
                "https://www.googleapis.com/auth/youtube.readonly"
            }
            Scope::YoutubeUpload => {
                "https://www.googleapis.com/auth/youtube.upload"
            }
            Scope::Youtubepartner => {
                "https://www.googleapis.com/auth/youtubepartner"
            }
            Scope::YoutubepartnerChannelAudit => {
                "https://www.googleapis.com/auth/youtubepartner-channel-audit"
            }
            Scope::YtAnalyticsMonetaryReadonly => {
                "https://www.googleapis.com/auth/yt-analytics-monetary.readonly"
            }
            Scope::YtAnalyticsReadonly => {
                "https://www.googleapis.com/auth/yt-analytics.readonly"
            }
            Scope::Custom(scope) => scope
        }
    }
    /// Return Scope assocaited with url
    pub fn scope(url: &str) -> Option<Scope> {
        match url {
            "https://www.googleapis.com/auth/activity" => Some(Scope::Activity),
            "https://www.googleapis.com/auth/adexchange.buyer" => Some(
                Scope::AdexchangeBuyer,
            ),
            "https://www.googleapis.com/auth/adexchange.seller" => Some(
                Scope::AdexchangeSeller,
            ),
            "https://www.googleapis.com/auth/adexchange.seller.readonly" => {
                Some(Scope::AdexchangeSellerReadonly)
            }
            "https://www.googleapis.com/auth/admin.datatransfer" => Some(
                Scope::AdminDatatransfer,
            ),
            "https://www.googleapis.com/auth/admin.datatransfer.readonly" => {
                Some(Scope::AdminDatatransferReadonly)
            }
            "https://www.googleapis.com/auth/admin.directory.customer" => {
                Some(Scope::AdminDirectoryCustomer)
            }
            "https://www.googleapis.com/auth/admin.directory.customer.readonly" => {
                Some(Scope::AdminDirectoryCustomerReadonly)
            }
            "https://www.googleapis.com/auth/admin.directory.device.chromeos" => {
                Some(Scope::AdminDirectoryDeviceChromeos)
            }
            "https://www.googleapis.com/auth/admin.directory.device.chromeos.readonly" => {
                Some(Scope::AdminDirectoryDeviceChromeosReadonly)
            }
            "https://www.googleapis.com/auth/admin.directory.device.mobile" => {
                Some(Scope::AdminDirectoryDeviceMobile)
            }
            "https://www.googleapis.com/auth/admin.directory.device.mobile.action" => {
                Some(Scope::AdminDirectoryDeviceMobileAction)
            }
            "https://www.googleapis.com/auth/admin.directory.device.mobile.readonly" => {
                Some(Scope::AdminDirectoryDeviceMobileReadonly)
            }
            "https://www.googleapis.com/auth/admin.directory.domain" => Some(
                Scope::AdminDirectoryDomain,
            ),
            "https://www.googleapis.com/auth/admin.directory.domain.readonly" => {
                Some(Scope::AdminDirectoryDomainReadonly)
            }
            "https://www.googleapis.com/auth/admin.directory.group" => Some(
                Scope::AdminDirectoryGroup,
            ),
            "https://www.googleapis.com/auth/admin.directory.group.member" => {
                Some(Scope::AdminDirectoryGroupMember)
            }
            "https://www.googleapis.com/auth/admin.directory.group.member.readonly" => {
                Some(Scope::AdminDirectoryGroupMemberReadonly)
            }
            "https://www.googleapis.com/auth/admin.directory.group.readonly" => {
                Some(Scope::AdminDirectoryGroupReadonly)
            }
            "https://www.googleapis.com/auth/admin.directory.notifications" => {
                Some(Scope::AdminDirectoryNotifications)
            }
            "https://www.googleapis.com/auth/admin.directory.orgunit" => Some(
                Scope::AdminDirectoryOrgunit,
            ),
            "https://www.googleapis.com/auth/admin.directory.orgunit.readonly" => {
                Some(Scope::AdminDirectoryOrgunitReadonly)
            }
            "https://www.googleapis.com/auth/admin.directory.resource.calendar" => {
                Some(Scope::AdminDirectoryResourceCalendar)
            }
            "https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly" => {
                Some(Scope::AdminDirectoryResourceCalendarReadonly)
            }
            "https://www.googleapis.com/auth/admin.directory.rolemanagement" => {
                Some(Scope::AdminDirectoryRolemanagement)
            }
            "https://www.googleapis.com/auth/admin.directory.rolemanagement.readonly" => {
                Some(Scope::AdminDirectoryRolemanagementReadonly)
            }
            "https://www.googleapis.com/auth/admin.directory.user" => Some(
                Scope::AdminDirectoryUser,
            ),
            "https://www.googleapis.com/auth/admin.directory.user.alias" => {
                Some(Scope::AdminDirectoryUserAlias)
            }
            "https://www.googleapis.com/auth/admin.directory.user.alias.readonly" => {
                Some(Scope::AdminDirectoryUserAliasReadonly)
            }
            "https://www.googleapis.com/auth/admin.directory.user.readonly" => {
                Some(Scope::AdminDirectoryUserReadonly)
            }
            "https://www.googleapis.com/auth/admin.directory.user.security" => {
                Some(Scope::AdminDirectoryUserSecurity)
            }
            "https://www.googleapis.com/auth/admin.directory.userschema" => {
                Some(Scope::AdminDirectoryUserschema)
            }
            "https://www.googleapis.com/auth/admin.directory.userschema.readonly" => {
                Some(Scope::AdminDirectoryUserschemaReadonly)
            }
            "https://www.googleapis.com/auth/admin.reports.audit.readonly" => {
                Some(Scope::AdminReportsAuditReadonly)
            }
            "https://www.googleapis.com/auth/admin.reports.usage.readonly" => {
                Some(Scope::AdminReportsUsageReadonly)
            }
            "https://www.googleapis.com/auth/adsense" => Some(Scope::Adsense),
            "https://www.googleapis.com/auth/adsense.readonly" => Some(
                Scope::AdsenseReadonly,
            ),
            "https://www.googleapis.com/auth/adsensehost" => Some(
                Scope::Adsensehost,
            ),
            "https://www.googleapis.com/auth/analytics" => Some(
                Scope::Analytics,
            ),
            "https://www.googleapis.com/auth/analytics.edit" => Some(
                Scope::AnalyticsEdit,
            ),
            "https://www.googleapis.com/auth/analytics.manage.users" => Some(
                Scope::AnalyticsManageUsers,
            ),
            "https://www.googleapis.com/auth/analytics.manage.users.readonly" => {
                Some(Scope::AnalyticsManageUsersReadonly)
            }
            "https://www.googleapis.com/auth/analytics.provision" => Some(
                Scope::AnalyticsProvision,
            ),
            "https://www.googleapis.com/auth/analytics.readonly" => Some(
                Scope::AnalyticsReadonly,
            ),
            "https://www.googleapis.com/auth/androidenterprise" => Some(
                Scope::Androidenterprise,
            ),
            "https://www.googleapis.com/auth/androidpublisher" => Some(
                Scope::Androidpublisher,
            ),
            "https://www.googleapis.com/auth/appengine.admin" => Some(
                Scope::AppengineAdmin,
            ),
            "https://www.googleapis.com/auth/apps.groups.migration" => Some(
                Scope::AppsGroupsMigration,
            ),
            "https://www.googleapis.com/auth/apps.groups.settings" => Some(
                Scope::AppsGroupsSettings,
            ),
            "https://www.googleapis.com/auth/apps.licensing" => Some(
                Scope::AppsLicensing,
            ),
            "https://www.googleapis.com/auth/apps.order" => Some(
                Scope::AppsOrder,
            ),
            "https://www.googleapis.com/auth/apps.order.readonly" => Some(
                Scope::AppsOrderReadonly,
            ),
            "https://www.googleapis.com/auth/appstate" => Some(Scope::Appstate),
            "https://www.googleapis.com/auth/bigquery" => Some(Scope::Bigquery),
            "https://www.googleapis.com/auth/bigquery.insertdata" => Some(
                Scope::BigqueryInsertdata,
            ),
            "https://www.googleapis.com/auth/blogger" => Some(Scope::Blogger),
            "https://www.googleapis.com/auth/blogger.readonly" => Some(
                Scope::BloggerReadonly,
            ),
            "https://www.googleapis.com/auth/books" => Some(Scope::Books),
            "https://www.googleapis.com/auth/calendar" => Some(Scope::Calendar),
            "https://www.google.com/calendar/feeds" => Some(
                Scope::CalendarFeeds,
            ),
            "https://www.googleapis.com/auth/calendar.readonly" => Some(
                Scope::CalendarReadonly,
            ),
            "https://www.googleapis.com/auth/classroom.courses" => Some(
                Scope::ClassroomCourses,
            ),
            "https://www.googleapis.com/auth/classroom.courses.readonly" => {
                Some(Scope::ClassroomCoursesReadonly)
            }
            "https://www.googleapis.com/auth/classroom.coursework.me" => Some(
                Scope::ClassroomCourseworkMe,
            ),
            "https://www.googleapis.com/auth/classroom.coursework.me.readonly" => {
                Some(Scope::ClassroomCourseworkMeReadonly)
            }
            "https://www.googleapis.com/auth/classroom.coursework.students" => {
                Some(Scope::ClassroomCourseworkStudents)
            }
            "https://www.googleapis.com/auth/classroom.coursework.students.readonly" => {
                Some(Scope::ClassroomCourseworkStudentsReadonly)
            }
            "https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly" => {
                Some(Scope::ClassroomGuardianlinksMeReadonly)
            }
            "https://www.googleapis.com/auth/classroom.guardianlinks.students" => {
                Some(Scope::ClassroomGuardianlinksStudents)
            }
            "https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly" => {
                Some(Scope::ClassroomGuardianlinksStudentsReadonly)
            }
            "https://www.googleapis.com/auth/classroom.profile.emails" => {
                Some(Scope::ClassroomProfileEmails)
            }
            "https://www.googleapis.com/auth/classroom.profile.photos" => {
                Some(Scope::ClassroomProfilePhotos)
            }
            "https://www.googleapis.com/auth/classroom.rosters" => Some(
                Scope::ClassroomRosters,
            ),
            "https://www.googleapis.com/auth/classroom.rosters.readonly" => {
                Some(Scope::ClassroomRostersReadonly)
            }
            "https://www.googleapis.com/auth/classroom.student-submissions.me.readonly" => {
                Some(Scope::ClassroomStudentSubmissionsMeReadonly)
            }
            "https://www.googleapis.com/auth/classroom.student-submissions.students.readonly" => {
                Some(Scope::ClassroomStudentSubmissionsStudentsReadonly)
            }
            "https://www.googleapis.com/auth/cloud_debugger" => Some(
                Scope::CloudDebugger,
            ),
            "https://www.googleapis.com/auth/cloud-language" => Some(
                Scope::CloudLanguage,
            ),
            "https://www.googleapis.com/auth/cloud-platform" => Some(
                Scope::CloudPlatform,
            ),
            "https://www.googleapis.com/auth/cloud-platform.read-only" => {
                Some(Scope::CloudPlatformReadOnly)
            }
            "https://www.googleapis.com/auth/cloud-translation" => Some(
                Scope::CloudTranslation,
            ),
            "https://www.googleapis.com/auth/cloud.useraccounts" => Some(
                Scope::CloudUseraccounts,
            ),
            "https://www.googleapis.com/auth/cloud.useraccounts.readonly" => {
                Some(Scope::CloudUseraccountsReadonly)
            }
            "https://www.googleapis.com/auth/cloudruntimeconfig" => Some(
                Scope::Cloudruntimeconfig,
            ),
            "https://www.googleapis.com/auth/compute" => Some(Scope::Compute),
            "https://www.googleapis.com/auth/compute.readonly" => Some(
                Scope::ComputeReadonly,
            ),
            "https://www.googleapis.com/auth/contacts" => Some(Scope::Contacts),
            "https://www.googleapis.com/auth/contacts.readonly" => Some(
                Scope::ContactsReadonly,
            ),
            "https://www.googleapis.com/auth/content" => Some(Scope::Content),
            "https://www.googleapis.com/auth/datastore" => Some(
                Scope::Datastore,
            ),
            "https://www.googleapis.com/auth/ddmconversions" => Some(
                Scope::Ddmconversions,
            ),
            "https://www.googleapis.com/auth/devstorage.full_control" => Some(
                Scope::DevstorageFullControl,
            ),
            "https://www.googleapis.com/auth/devstorage.read_only" => Some(
                Scope::DevstorageReadOnly,
            ),
            "https://www.googleapis.com/auth/devstorage.read_write" => Some(
                Scope::DevstorageReadWrite,
            ),
            "https://www.googleapis.com/auth/dfareporting" => Some(
                Scope::Dfareporting,
            ),
            "https://www.googleapis.com/auth/dfatrafficking" => Some(
                Scope::Dfatrafficking,
            ),
            "https://www.googleapis.com/auth/doubleclickbidmanager" => Some(
                Scope::Doubleclickbidmanager,
            ),
            "https://www.googleapis.com/auth/doubleclicksearch" => Some(
                Scope::Doubleclicksearch,
            ),
            "https://www.googleapis.com/auth/drive" => Some(Scope::Drive),
            "https://www.googleapis.com/auth/drive.appdata" => Some(
                Scope::DriveAppdata,
            ),
            "https://www.googleapis.com/auth/drive.file" => Some(
                Scope::DriveFile,
            ),
            "https://www.googleapis.com/auth/drive.metadata" => Some(
                Scope::DriveMetadata,
            ),
            "https://www.googleapis.com/auth/drive.metadata.readonly" => Some(
                Scope::DriveMetadataReadonly,
            ),
            "https://www.googleapis.com/auth/drive.photos.readonly" => Some(
                Scope::DrivePhotosReadonly,
            ),
            "https://www.googleapis.com/auth/drive.readonly" => Some(
                Scope::DriveReadonly,
            ),
            "https://www.googleapis.com/auth/drive.scripts" => Some(
                Scope::DriveScripts,
            ),
            "https://www.googleapis.com/auth/firebase" => Some(Scope::Firebase),
            "https://www.googleapis.com/auth/firebase.readonly" => Some(
                Scope::FirebaseReadonly,
            ),
            "https://www.googleapis.com/auth/fitness.activity.read" => Some(
                Scope::FitnessActivityRead,
            ),
            "https://www.googleapis.com/auth/fitness.activity.write" => Some(
                Scope::FitnessActivityWrite,
            ),
            "https://www.googleapis.com/auth/fitness.blood_glucose.read" => {
                Some(Scope::FitnessBloodGlucoseRead)
            }
            "https://www.googleapis.com/auth/fitness.blood_glucose.write" => {
                Some(Scope::FitnessBloodGlucoseWrite)
            }
            "https://www.googleapis.com/auth/fitness.blood_pressure.read" => {
                Some(Scope::FitnessBloodPressureRead)
            }
            "https://www.googleapis.com/auth/fitness.blood_pressure.write" => {
                Some(Scope::FitnessBloodPressureWrite)
            }
            "https://www.googleapis.com/auth/fitness.body.read" => Some(
                Scope::FitnessBodyRead,
            ),
            "https://www.googleapis.com/auth/fitness.body_temperature.read" => {
                Some(Scope::FitnessBodyTemperatureRead)
            }
            "https://www.googleapis.com/auth/fitness.body_temperature.write" => {
                Some(Scope::FitnessBodyTemperatureWrite)
            }
            "https://www.googleapis.com/auth/fitness.body.write" => Some(
                Scope::FitnessBodyWrite,
            ),
            "https://www.googleapis.com/auth/fitness.location.read" => Some(
                Scope::FitnessLocationRead,
            ),
            "https://www.googleapis.com/auth/fitness.location.write" => Some(
                Scope::FitnessLocationWrite,
            ),
            "https://www.googleapis.com/auth/fitness.nutrition.read" => Some(
                Scope::FitnessNutritionRead,
            ),
            "https://www.googleapis.com/auth/fitness.nutrition.write" => Some(
                Scope::FitnessNutritionWrite,
            ),
            "https://www.googleapis.com/auth/fitness.oxygen_saturation.read" => {
                Some(Scope::FitnessOxygenSaturationRead)
            }
            "https://www.googleapis.com/auth/fitness.oxygen_saturation.write" => {
                Some(Scope::FitnessOxygenSaturationWrite)
            }
            "https://www.googleapis.com/auth/fitness.reproductive_health.read" => {
                Some(Scope::FitnessReproductiveHealthRead)
            }
            "https://www.googleapis.com/auth/fitness.reproductive_health.write" => {
                Some(Scope::FitnessReproductiveHealthWrite)
            }
            "https://www.googleapis.com/auth/forms" => Some(Scope::Forms),
            "https://www.googleapis.com/auth/forms.currentonly" => Some(
                Scope::FormsCurrentonly,
            ),
            "https://www.googleapis.com/auth/fusiontables" => Some(
                Scope::Fusiontables,
            ),
            "https://www.googleapis.com/auth/fusiontables.readonly" => Some(
                Scope::FusiontablesReadonly,
            ),
            "https://www.googleapis.com/auth/games" => Some(Scope::Games),
            "https://www.googleapis.com/auth/genomics" => Some(Scope::Genomics),
            "https://www.googleapis.com/auth/genomics.readonly" => Some(
                Scope::GenomicsReadonly,
            ),
            "https://www.googleapis.com/auth/glass.location" => Some(
                Scope::GlassLocation,
            ),
            "https://www.googleapis.com/auth/glass.timeline" => Some(
                Scope::GlassTimeline,
            ),
            "https://mail.google.com/" => Some(Scope::Gmail),
            "https://www.googleapis.com/auth/gmail.compose" => Some(
                Scope::GmailCompose,
            ),
            "https://www.googleapis.com/auth/gmail.insert" => Some(
                Scope::GmailInsert,
            ),
            "https://www.googleapis.com/auth/gmail.labels" => Some(
                Scope::GmailLabels,
            ),
            "https://www.googleapis.com/auth/gmail.metadata" => Some(
                Scope::GmailMetadata,
            ),
            "https://www.googleapis.com/auth/gmail.modify" => Some(
                Scope::GmailModify,
            ),
            "https://www.googleapis.com/auth/gmail.readonly" => Some(
                Scope::GmailReadonly,
            ),
            "https://www.googleapis.com/auth/gmail.send" => Some(
                Scope::GmailSend,
            ),
            "https://www.googleapis.com/auth/gmail.settings.basic" => Some(
                Scope::GmailSettingsBasic,
            ),
            "https://www.googleapis.com/auth/gmail.settings.sharing" => Some(
                Scope::GmailSettingsSharing,
            ),
            "https://www.googleapis.com/auth/groups" => Some(Scope::Groups),
            "https://www.googleapis.com/auth/logging.admin" => Some(
                Scope::LoggingAdmin,
            ),
            "https://www.googleapis.com/auth/logging.read" => Some(
                Scope::LoggingRead,
            ),
            "https://www.googleapis.com/auth/logging.write" => Some(
                Scope::LoggingWrite,
            ),
            "https://www.google.com/m8/feeds" => Some(Scope::M8Feeds),
            "https://www.googleapis.com/auth/manufacturercenter" => Some(
                Scope::Manufacturercenter,
            ),
            "https://www.googleapis.com/auth/monitoring" => Some(
                Scope::Monitoring,
            ),
            "https://www.googleapis.com/auth/monitoring.read" => Some(
                Scope::MonitoringRead,
            ),
            "https://www.googleapis.com/auth/monitoring.write" => Some(
                Scope::MonitoringWrite,
            ),
            "https://www.googleapis.com/auth/ndev.clouddns.readonly" => Some(
                Scope::NdevClouddnsReadonly,
            ),
            "https://www.googleapis.com/auth/ndev.clouddns.readwrite" => Some(
                Scope::NdevClouddnsReadwrite,
            ),
            "https://www.googleapis.com/auth/ndev.cloudman" => Some(
                Scope::NdevCloudman,
            ),
            "https://www.googleapis.com/auth/ndev.cloudman.readonly" => Some(
                Scope::NdevCloudmanReadonly,
            ),
            "https://www.googleapis.com/auth/playmovies_partner.readonly" => {
                Some(Scope::PlaymoviesPartnerReadonly)
            }
            "https://www.googleapis.com/auth/plus.circles.read" => Some(
                Scope::PlusCirclesRead,
            ),
            "https://www.googleapis.com/auth/plus.circles.write" => Some(
                Scope::PlusCirclesWrite,
            ),
            "https://www.googleapis.com/auth/plus.login" => Some(
                Scope::PlusLogin,
            ),
            "https://www.googleapis.com/auth/plus.me" => Some(Scope::PlusMe),
            "https://www.googleapis.com/auth/plus.media.upload" => Some(
                Scope::PlusMediaUpload,
            ),
            "https://www.googleapis.com/auth/plus.profiles.read" => Some(
                Scope::PlusProfilesRead,
            ),
            "https://www.googleapis.com/auth/plus.stream.read" => Some(
                Scope::PlusStreamRead,
            ),
            "https://www.googleapis.com/auth/plus.stream.write" => Some(
                Scope::PlusStreamWrite,
            ),
            "https://www.googleapis.com/auth/prediction" => Some(
                Scope::Prediction,
            ),
            "https://www.googleapis.com/auth/presentations" => Some(
                Scope::Presentations,
            ),
            "https://www.googleapis.com/auth/presentations.readonly" => Some(
                Scope::PresentationsReadonly,
            ),
            "https://www.googleapis.com/auth/pubsub" => Some(Scope::Pubsub),
            "https://www.googleapis.com/auth/replicapool" => Some(
                Scope::Replicapool,
            ),
            "https://www.googleapis.com/auth/replicapool.readonly" => Some(
                Scope::ReplicapoolReadonly,
            ),
            "https://www.googleapis.com/auth/service.management" => Some(
                Scope::ServiceManagement,
            ),
            "https://www.googleapis.com/auth/service.management.readonly" => {
                Some(Scope::ServiceManagementReadonly)
            }
            "https://www.googleapis.com/auth/servicecontrol" => Some(
                Scope::Servicecontrol,
            ),
            "https://www.googleapis.com/auth/siteverification" => Some(
                Scope::Siteverification,
            ),
            "https://www.googleapis.com/auth/siteverification.verify_only" => {
                Some(Scope::SiteverificationVerifyOnly)
            }
            "https://www.googleapis.com/auth/source.read_only" => Some(
                Scope::SourceReadOnly,
            ),
            "https://www.googleapis.com/auth/source.read_write" => Some(
                Scope::SourceReadWrite,
            ),
            "https://www.googleapis.com/auth/spanner.admin" => Some(
                Scope::SpannerAdmin,
            ),
            "https://www.googleapis.com/auth/spanner.data" => Some(
                Scope::SpannerData,
            ),
            "https://www.googleapis.com/auth/spreadsheets" => Some(
                Scope::Spreadsheets,
            ),
            "https://www.googleapis.com/auth/spreadsheets.readonly" => Some(
                Scope::SpreadsheetsReadonly,
            ),
            "https://www.googleapis.com/auth/sqlservice.admin" => Some(
                Scope::SqlserviceAdmin,
            ),
            "https://www.googleapis.com/auth/tagmanager.delete.containers" => {
                Some(Scope::TagmanagerDeleteContainers)
            }
            "https://www.googleapis.com/auth/tagmanager.edit.containers" => {
                Some(Scope::TagmanagerEditContainers)
            }
            "https://www.googleapis.com/auth/tagmanager.edit.containerversions" => {
                Some(Scope::TagmanagerEditContainerversions)
            }
            "https://www.googleapis.com/auth/tagmanager.manage.accounts" => {
                Some(Scope::TagmanagerManageAccounts)
            }
            "https://www.googleapis.com/auth/tagmanager.manage.users" => Some(
                Scope::TagmanagerManageUsers,
            ),
            "https://www.googleapis.com/auth/tagmanager.publish" => Some(
                Scope::TagmanagerPublish,
            ),
            "https://www.googleapis.com/auth/tagmanager.readonly" => Some(
                Scope::TagmanagerReadonly,
            ),
            "https://www.googleapis.com/auth/taskqueue" => Some(
                Scope::Taskqueue,
            ),
            "https://www.googleapis.com/auth/taskqueue.consumer" => Some(
                Scope::TaskqueueConsumer,
            ),
            "https://www.googleapis.com/auth/tasks" => Some(Scope::Tasks),
            "https://www.googleapis.com/auth/tasks.readonly" => Some(
                Scope::TasksReadonly,
            ),
            "https://www.googleapis.com/auth/urlshortener" => Some(
                Scope::Urlshortener,
            ),
            "https://www.googleapis.com/auth/user.addresses.read" => Some(
                Scope::UserAddressesRead,
            ),
            "https://www.googleapis.com/auth/user.birthday.read" => Some(
                Scope::UserBirthdayRead,
            ),
            "https://www.googleapis.com/auth/user.emails.read" => Some(
                Scope::UserEmailsRead,
            ),
            "https://www.googleapis.com/auth/user.phonenumbers.read" => Some(
                Scope::UserPhonenumbersRead,
            ),
            "https://www.googleapis.com/auth/userinfo.email" => Some(
                Scope::UserinfoEmail,
            ),
            "https://www.googleapis.com/auth/userinfo.profile" => Some(
                Scope::UserinfoProfile,
            ),
            "https://www.googleapis.com/auth/userlocation.beacon.registry" => {
                Some(Scope::UserlocationBeaconRegistry)
            }
            "https://www.googleapis.com/auth/webmasters" => Some(
                Scope::Webmasters,
            ),
            "https://www.googleapis.com/auth/webmasters.readonly" => Some(
                Scope::WebmastersReadonly,
            ),
            "https://www.googleapis.com/auth/xapi.zoo" => Some(Scope::XapiZoo),
            "https://www.googleapis.com/auth/youtube" => Some(Scope::Youtube),
            "https://www.googleapis.com/auth/youtube.force-ssl" => Some(
                Scope::YoutubeForceSsl,
            ),
            "https://www.googleapis.com/auth/youtube.readonly" => Some(
                Scope::YoutubeReadonly,
            ),
            "https://www.googleapis.com/auth/youtube.upload" => Some(
                Scope::YoutubeUpload,
            ),
            "https://www.googleapis.com/auth/youtubepartner" => Some(
                Scope::Youtubepartner,
            ),
            "https://www.googleapis.com/auth/youtubepartner-channel-audit" => {
                Some(Scope::YoutubepartnerChannelAudit)
            }
            "https://www.googleapis.com/auth/yt-analytics-monetary.readonly" => {
                Some(Scope::YtAnalyticsMonetaryReadonly)
            }
            "https://www.googleapis.com/auth/yt-analytics.readonly" => Some(
                Scope::YtAnalyticsReadonly,
            ),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_activity() {
        assert_eq!("https://www.googleapis.com/auth/activity", Scope::Activity.url());
        assert_eq!("https://www.googleapis.com/auth/activity".parse::<Scope>().unwrap(), Scope::Activity);
    }
    #[test]
    fn test_adexchangebuyer() {
        assert_eq!("https://www.googleapis.com/auth/adexchange.buyer", Scope::AdexchangeBuyer.url());
        assert_eq!("https://www.googleapis.com/auth/adexchange.buyer".parse::<Scope>().unwrap(), Scope::AdexchangeBuyer);
    }
    #[test]
    fn test_adexchangeseller() {
        assert_eq!("https://www.googleapis.com/auth/adexchange.seller", Scope::AdexchangeSeller.url());
        assert_eq!("https://www.googleapis.com/auth/adexchange.seller".parse::<Scope>().unwrap(), Scope::AdexchangeSeller);
    }
    #[test]
    fn test_adexchangesellerreadonly() {
        assert_eq!("https://www.googleapis.com/auth/adexchange.seller.readonly", Scope::AdexchangeSellerReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/adexchange.seller.readonly".parse::<Scope>().unwrap(), Scope::AdexchangeSellerReadonly);
    }
    #[test]
    fn test_admindatatransfer() {
        assert_eq!("https://www.googleapis.com/auth/admin.datatransfer", Scope::AdminDatatransfer.url());
        assert_eq!("https://www.googleapis.com/auth/admin.datatransfer".parse::<Scope>().unwrap(), Scope::AdminDatatransfer);
    }
    #[test]
    fn test_admindatatransferreadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.datatransfer.readonly", Scope::AdminDatatransferReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.datatransfer.readonly".parse::<Scope>().unwrap(), Scope::AdminDatatransferReadonly);
    }
    #[test]
    fn test_admindirectorycustomer() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.customer", Scope::AdminDirectoryCustomer.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.customer".parse::<Scope>().unwrap(), Scope::AdminDirectoryCustomer);
    }
    #[test]
    fn test_admindirectorycustomerreadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.customer.readonly", Scope::AdminDirectoryCustomerReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.customer.readonly".parse::<Scope>().unwrap(), Scope::AdminDirectoryCustomerReadonly);
    }
    #[test]
    fn test_admindirectorydevicechromeos() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.device.chromeos", Scope::AdminDirectoryDeviceChromeos.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.device.chromeos".parse::<Scope>().unwrap(), Scope::AdminDirectoryDeviceChromeos);
    }
    #[test]
    fn test_admindirectorydevicechromeosreadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.device.chromeos.readonly", Scope::AdminDirectoryDeviceChromeosReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.device.chromeos.readonly".parse::<Scope>().unwrap(), Scope::AdminDirectoryDeviceChromeosReadonly);
    }
    #[test]
    fn test_admindirectorydevicemobile() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.device.mobile", Scope::AdminDirectoryDeviceMobile.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.device.mobile".parse::<Scope>().unwrap(), Scope::AdminDirectoryDeviceMobile);
    }
    #[test]
    fn test_admindirectorydevicemobileaction() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.device.mobile.action", Scope::AdminDirectoryDeviceMobileAction.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.device.mobile.action".parse::<Scope>().unwrap(), Scope::AdminDirectoryDeviceMobileAction);
    }
    #[test]
    fn test_admindirectorydevicemobilereadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.device.mobile.readonly", Scope::AdminDirectoryDeviceMobileReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.device.mobile.readonly".parse::<Scope>().unwrap(), Scope::AdminDirectoryDeviceMobileReadonly);
    }
    #[test]
    fn test_admindirectorydomain() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.domain", Scope::AdminDirectoryDomain.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.domain".parse::<Scope>().unwrap(), Scope::AdminDirectoryDomain);
    }
    #[test]
    fn test_admindirectorydomainreadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.domain.readonly", Scope::AdminDirectoryDomainReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.domain.readonly".parse::<Scope>().unwrap(), Scope::AdminDirectoryDomainReadonly);
    }
    #[test]
    fn test_admindirectorygroup() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.group", Scope::AdminDirectoryGroup.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.group".parse::<Scope>().unwrap(), Scope::AdminDirectoryGroup);
    }
    #[test]
    fn test_admindirectorygroupmember() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.group.member", Scope::AdminDirectoryGroupMember.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.group.member".parse::<Scope>().unwrap(), Scope::AdminDirectoryGroupMember);
    }
    #[test]
    fn test_admindirectorygroupmemberreadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.group.member.readonly", Scope::AdminDirectoryGroupMemberReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.group.member.readonly".parse::<Scope>().unwrap(), Scope::AdminDirectoryGroupMemberReadonly);
    }
    #[test]
    fn test_admindirectorygroupreadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.group.readonly", Scope::AdminDirectoryGroupReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.group.readonly".parse::<Scope>().unwrap(), Scope::AdminDirectoryGroupReadonly);
    }
    #[test]
    fn test_admindirectorynotifications() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.notifications", Scope::AdminDirectoryNotifications.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.notifications".parse::<Scope>().unwrap(), Scope::AdminDirectoryNotifications);
    }
    #[test]
    fn test_admindirectoryorgunit() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.orgunit", Scope::AdminDirectoryOrgunit.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.orgunit".parse::<Scope>().unwrap(), Scope::AdminDirectoryOrgunit);
    }
    #[test]
    fn test_admindirectoryorgunitreadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.orgunit.readonly", Scope::AdminDirectoryOrgunitReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.orgunit.readonly".parse::<Scope>().unwrap(), Scope::AdminDirectoryOrgunitReadonly);
    }
    #[test]
    fn test_admindirectoryresourcecalendar() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.resource.calendar", Scope::AdminDirectoryResourceCalendar.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.resource.calendar".parse::<Scope>().unwrap(), Scope::AdminDirectoryResourceCalendar);
    }
    #[test]
    fn test_admindirectoryresourcecalendarreadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly", Scope::AdminDirectoryResourceCalendarReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly".parse::<Scope>().unwrap(), Scope::AdminDirectoryResourceCalendarReadonly);
    }
    #[test]
    fn test_admindirectoryrolemanagement() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.rolemanagement", Scope::AdminDirectoryRolemanagement.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.rolemanagement".parse::<Scope>().unwrap(), Scope::AdminDirectoryRolemanagement);
    }
    #[test]
    fn test_admindirectoryrolemanagementreadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.rolemanagement.readonly", Scope::AdminDirectoryRolemanagementReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.rolemanagement.readonly".parse::<Scope>().unwrap(), Scope::AdminDirectoryRolemanagementReadonly);
    }
    #[test]
    fn test_admindirectoryuser() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.user", Scope::AdminDirectoryUser.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.user".parse::<Scope>().unwrap(), Scope::AdminDirectoryUser);
    }
    #[test]
    fn test_admindirectoryuseralias() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.user.alias", Scope::AdminDirectoryUserAlias.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.user.alias".parse::<Scope>().unwrap(), Scope::AdminDirectoryUserAlias);
    }
    #[test]
    fn test_admindirectoryuseraliasreadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.user.alias.readonly", Scope::AdminDirectoryUserAliasReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.user.alias.readonly".parse::<Scope>().unwrap(), Scope::AdminDirectoryUserAliasReadonly);
    }
    #[test]
    fn test_admindirectoryuserreadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.user.readonly", Scope::AdminDirectoryUserReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.user.readonly".parse::<Scope>().unwrap(), Scope::AdminDirectoryUserReadonly);
    }
    #[test]
    fn test_admindirectoryusersecurity() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.user.security", Scope::AdminDirectoryUserSecurity.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.user.security".parse::<Scope>().unwrap(), Scope::AdminDirectoryUserSecurity);
    }
    #[test]
    fn test_admindirectoryuserschema() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.userschema", Scope::AdminDirectoryUserschema.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.userschema".parse::<Scope>().unwrap(), Scope::AdminDirectoryUserschema);
    }
    #[test]
    fn test_admindirectoryuserschemareadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.directory.userschema.readonly", Scope::AdminDirectoryUserschemaReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.directory.userschema.readonly".parse::<Scope>().unwrap(), Scope::AdminDirectoryUserschemaReadonly);
    }
    #[test]
    fn test_adminreportsauditreadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.reports.audit.readonly", Scope::AdminReportsAuditReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.reports.audit.readonly".parse::<Scope>().unwrap(), Scope::AdminReportsAuditReadonly);
    }
    #[test]
    fn test_adminreportsusagereadonly() {
        assert_eq!("https://www.googleapis.com/auth/admin.reports.usage.readonly", Scope::AdminReportsUsageReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/admin.reports.usage.readonly".parse::<Scope>().unwrap(), Scope::AdminReportsUsageReadonly);
    }
    #[test]
    fn test_adsense() {
        assert_eq!("https://www.googleapis.com/auth/adsense", Scope::Adsense.url());
        assert_eq!("https://www.googleapis.com/auth/adsense".parse::<Scope>().unwrap(), Scope::Adsense);
    }
    #[test]
    fn test_adsensereadonly() {
        assert_eq!("https://www.googleapis.com/auth/adsense.readonly", Scope::AdsenseReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/adsense.readonly".parse::<Scope>().unwrap(), Scope::AdsenseReadonly);
    }
    #[test]
    fn test_adsensehost() {
        assert_eq!("https://www.googleapis.com/auth/adsensehost", Scope::Adsensehost.url());
        assert_eq!("https://www.googleapis.com/auth/adsensehost".parse::<Scope>().unwrap(), Scope::Adsensehost);
    }
    #[test]
    fn test_analytics() {
        assert_eq!("https://www.googleapis.com/auth/analytics", Scope::Analytics.url());
        assert_eq!("https://www.googleapis.com/auth/analytics".parse::<Scope>().unwrap(), Scope::Analytics);
    }
    #[test]
    fn test_analyticsedit() {
        assert_eq!("https://www.googleapis.com/auth/analytics.edit", Scope::AnalyticsEdit.url());
        assert_eq!("https://www.googleapis.com/auth/analytics.edit".parse::<Scope>().unwrap(), Scope::AnalyticsEdit);
    }
    #[test]
    fn test_analyticsmanageusers() {
        assert_eq!("https://www.googleapis.com/auth/analytics.manage.users", Scope::AnalyticsManageUsers.url());
        assert_eq!("https://www.googleapis.com/auth/analytics.manage.users".parse::<Scope>().unwrap(), Scope::AnalyticsManageUsers);
    }
    #[test]
    fn test_analyticsmanageusersreadonly() {
        assert_eq!("https://www.googleapis.com/auth/analytics.manage.users.readonly", Scope::AnalyticsManageUsersReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/analytics.manage.users.readonly".parse::<Scope>().unwrap(), Scope::AnalyticsManageUsersReadonly);
    }
    #[test]
    fn test_analyticsprovision() {
        assert_eq!("https://www.googleapis.com/auth/analytics.provision", Scope::AnalyticsProvision.url());
        assert_eq!("https://www.googleapis.com/auth/analytics.provision".parse::<Scope>().unwrap(), Scope::AnalyticsProvision);
    }
    #[test]
    fn test_analyticsreadonly() {
        assert_eq!("https://www.googleapis.com/auth/analytics.readonly", Scope::AnalyticsReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/analytics.readonly".parse::<Scope>().unwrap(), Scope::AnalyticsReadonly);
    }
    #[test]
    fn test_androidenterprise() {
        assert_eq!("https://www.googleapis.com/auth/androidenterprise", Scope::Androidenterprise.url());
        assert_eq!("https://www.googleapis.com/auth/androidenterprise".parse::<Scope>().unwrap(), Scope::Androidenterprise);
    }
    #[test]
    fn test_androidpublisher() {
        assert_eq!("https://www.googleapis.com/auth/androidpublisher", Scope::Androidpublisher.url());
        assert_eq!("https://www.googleapis.com/auth/androidpublisher".parse::<Scope>().unwrap(), Scope::Androidpublisher);
    }
    #[test]
    fn test_appengineadmin() {
        assert_eq!("https://www.googleapis.com/auth/appengine.admin", Scope::AppengineAdmin.url());
        assert_eq!("https://www.googleapis.com/auth/appengine.admin".parse::<Scope>().unwrap(), Scope::AppengineAdmin);
    }
    #[test]
    fn test_appsgroupsmigration() {
        assert_eq!("https://www.googleapis.com/auth/apps.groups.migration", Scope::AppsGroupsMigration.url());
        assert_eq!("https://www.googleapis.com/auth/apps.groups.migration".parse::<Scope>().unwrap(), Scope::AppsGroupsMigration);
    }
    #[test]
    fn test_appsgroupssettings() {
        assert_eq!("https://www.googleapis.com/auth/apps.groups.settings", Scope::AppsGroupsSettings.url());
        assert_eq!("https://www.googleapis.com/auth/apps.groups.settings".parse::<Scope>().unwrap(), Scope::AppsGroupsSettings);
    }
    #[test]
    fn test_appslicensing() {
        assert_eq!("https://www.googleapis.com/auth/apps.licensing", Scope::AppsLicensing.url());
        assert_eq!("https://www.googleapis.com/auth/apps.licensing".parse::<Scope>().unwrap(), Scope::AppsLicensing);
    }
    #[test]
    fn test_appsorder() {
        assert_eq!("https://www.googleapis.com/auth/apps.order", Scope::AppsOrder.url());
        assert_eq!("https://www.googleapis.com/auth/apps.order".parse::<Scope>().unwrap(), Scope::AppsOrder);
    }
    #[test]
    fn test_appsorderreadonly() {
        assert_eq!("https://www.googleapis.com/auth/apps.order.readonly", Scope::AppsOrderReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/apps.order.readonly".parse::<Scope>().unwrap(), Scope::AppsOrderReadonly);
    }
    #[test]
    fn test_appstate() {
        assert_eq!("https://www.googleapis.com/auth/appstate", Scope::Appstate.url());
        assert_eq!("https://www.googleapis.com/auth/appstate".parse::<Scope>().unwrap(), Scope::Appstate);
    }
    #[test]
    fn test_bigquery() {
        assert_eq!("https://www.googleapis.com/auth/bigquery", Scope::Bigquery.url());
        assert_eq!("https://www.googleapis.com/auth/bigquery".parse::<Scope>().unwrap(), Scope::Bigquery);
    }
    #[test]
    fn test_bigqueryinsertdata() {
        assert_eq!("https://www.googleapis.com/auth/bigquery.insertdata", Scope::BigqueryInsertdata.url());
        assert_eq!("https://www.googleapis.com/auth/bigquery.insertdata".parse::<Scope>().unwrap(), Scope::BigqueryInsertdata);
    }
    #[test]
    fn test_blogger() {
        assert_eq!("https://www.googleapis.com/auth/blogger", Scope::Blogger.url());
        assert_eq!("https://www.googleapis.com/auth/blogger".parse::<Scope>().unwrap(), Scope::Blogger);
    }
    #[test]
    fn test_bloggerreadonly() {
        assert_eq!("https://www.googleapis.com/auth/blogger.readonly", Scope::BloggerReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/blogger.readonly".parse::<Scope>().unwrap(), Scope::BloggerReadonly);
    }
    #[test]
    fn test_books() {
        assert_eq!("https://www.googleapis.com/auth/books", Scope::Books.url());
        assert_eq!("https://www.googleapis.com/auth/books".parse::<Scope>().unwrap(), Scope::Books);
    }
    #[test]
    fn test_calendar() {
        assert_eq!("https://www.googleapis.com/auth/calendar", Scope::Calendar.url());
        assert_eq!("https://www.googleapis.com/auth/calendar".parse::<Scope>().unwrap(), Scope::Calendar);
    }
    #[test]
    fn test_calendarfeeds() {
        assert_eq!("https://www.google.com/calendar/feeds", Scope::CalendarFeeds.url());
        assert_eq!("https://www.google.com/calendar/feeds".parse::<Scope>().unwrap(), Scope::CalendarFeeds);
    }
    #[test]
    fn test_calendarreadonly() {
        assert_eq!("https://www.googleapis.com/auth/calendar.readonly", Scope::CalendarReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/calendar.readonly".parse::<Scope>().unwrap(), Scope::CalendarReadonly);
    }
    #[test]
    fn test_classroomcourses() {
        assert_eq!("https://www.googleapis.com/auth/classroom.courses", Scope::ClassroomCourses.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.courses".parse::<Scope>().unwrap(), Scope::ClassroomCourses);
    }
    #[test]
    fn test_classroomcoursesreadonly() {
        assert_eq!("https://www.googleapis.com/auth/classroom.courses.readonly", Scope::ClassroomCoursesReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.courses.readonly".parse::<Scope>().unwrap(), Scope::ClassroomCoursesReadonly);
    }
    #[test]
    fn test_classroomcourseworkme() {
        assert_eq!("https://www.googleapis.com/auth/classroom.coursework.me", Scope::ClassroomCourseworkMe.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.coursework.me".parse::<Scope>().unwrap(), Scope::ClassroomCourseworkMe);
    }
    #[test]
    fn test_classroomcourseworkmereadonly() {
        assert_eq!("https://www.googleapis.com/auth/classroom.coursework.me.readonly", Scope::ClassroomCourseworkMeReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.coursework.me.readonly".parse::<Scope>().unwrap(), Scope::ClassroomCourseworkMeReadonly);
    }
    #[test]
    fn test_classroomcourseworkstudents() {
        assert_eq!("https://www.googleapis.com/auth/classroom.coursework.students", Scope::ClassroomCourseworkStudents.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.coursework.students".parse::<Scope>().unwrap(), Scope::ClassroomCourseworkStudents);
    }
    #[test]
    fn test_classroomcourseworkstudentsreadonly() {
        assert_eq!("https://www.googleapis.com/auth/classroom.coursework.students.readonly", Scope::ClassroomCourseworkStudentsReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.coursework.students.readonly".parse::<Scope>().unwrap(), Scope::ClassroomCourseworkStudentsReadonly);
    }
    #[test]
    fn test_classroomguardianlinksmereadonly() {
        assert_eq!("https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly", Scope::ClassroomGuardianlinksMeReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly".parse::<Scope>().unwrap(), Scope::ClassroomGuardianlinksMeReadonly);
    }
    #[test]
    fn test_classroomguardianlinksstudents() {
        assert_eq!("https://www.googleapis.com/auth/classroom.guardianlinks.students", Scope::ClassroomGuardianlinksStudents.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.guardianlinks.students".parse::<Scope>().unwrap(), Scope::ClassroomGuardianlinksStudents);
    }
    #[test]
    fn test_classroomguardianlinksstudentsreadonly() {
        assert_eq!("https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly", Scope::ClassroomGuardianlinksStudentsReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly".parse::<Scope>().unwrap(), Scope::ClassroomGuardianlinksStudentsReadonly);
    }
    #[test]
    fn test_classroomprofileemails() {
        assert_eq!("https://www.googleapis.com/auth/classroom.profile.emails", Scope::ClassroomProfileEmails.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.profile.emails".parse::<Scope>().unwrap(), Scope::ClassroomProfileEmails);
    }
    #[test]
    fn test_classroomprofilephotos() {
        assert_eq!("https://www.googleapis.com/auth/classroom.profile.photos", Scope::ClassroomProfilePhotos.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.profile.photos".parse::<Scope>().unwrap(), Scope::ClassroomProfilePhotos);
    }
    #[test]
    fn test_classroomrosters() {
        assert_eq!("https://www.googleapis.com/auth/classroom.rosters", Scope::ClassroomRosters.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.rosters".parse::<Scope>().unwrap(), Scope::ClassroomRosters);
    }
    #[test]
    fn test_classroomrostersreadonly() {
        assert_eq!("https://www.googleapis.com/auth/classroom.rosters.readonly", Scope::ClassroomRostersReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.rosters.readonly".parse::<Scope>().unwrap(), Scope::ClassroomRostersReadonly);
    }
    #[test]
    fn test_classroomstudentsubmissionsmereadonly() {
        assert_eq!("https://www.googleapis.com/auth/classroom.student-submissions.me.readonly", Scope::ClassroomStudentSubmissionsMeReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.student-submissions.me.readonly".parse::<Scope>().unwrap(), Scope::ClassroomStudentSubmissionsMeReadonly);
    }
    #[test]
    fn test_classroomstudentsubmissionsstudentsreadonly() {
        assert_eq!("https://www.googleapis.com/auth/classroom.student-submissions.students.readonly", Scope::ClassroomStudentSubmissionsStudentsReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/classroom.student-submissions.students.readonly".parse::<Scope>().unwrap(), Scope::ClassroomStudentSubmissionsStudentsReadonly);
    }
    #[test]
    fn test_clouddebugger() {
        assert_eq!("https://www.googleapis.com/auth/cloud_debugger", Scope::CloudDebugger.url());
        assert_eq!("https://www.googleapis.com/auth/cloud_debugger".parse::<Scope>().unwrap(), Scope::CloudDebugger);
    }
    #[test]
    fn test_cloudlanguage() {
        assert_eq!("https://www.googleapis.com/auth/cloud-language", Scope::CloudLanguage.url());
        assert_eq!("https://www.googleapis.com/auth/cloud-language".parse::<Scope>().unwrap(), Scope::CloudLanguage);
    }
    #[test]
    fn test_cloudplatform() {
        assert_eq!("https://www.googleapis.com/auth/cloud-platform", Scope::CloudPlatform.url());
        assert_eq!("https://www.googleapis.com/auth/cloud-platform".parse::<Scope>().unwrap(), Scope::CloudPlatform);
    }
    #[test]
    fn test_cloudplatformreadonly() {
        assert_eq!("https://www.googleapis.com/auth/cloud-platform.read-only", Scope::CloudPlatformReadOnly.url());
        assert_eq!("https://www.googleapis.com/auth/cloud-platform.read-only".parse::<Scope>().unwrap(), Scope::CloudPlatformReadOnly);
    }
    #[test]
    fn test_cloudtranslation() {
        assert_eq!("https://www.googleapis.com/auth/cloud-translation", Scope::CloudTranslation.url());
        assert_eq!("https://www.googleapis.com/auth/cloud-translation".parse::<Scope>().unwrap(), Scope::CloudTranslation);
    }
    #[test]
    fn test_clouduseraccounts() {
        assert_eq!("https://www.googleapis.com/auth/cloud.useraccounts", Scope::CloudUseraccounts.url());
        assert_eq!("https://www.googleapis.com/auth/cloud.useraccounts".parse::<Scope>().unwrap(), Scope::CloudUseraccounts);
    }
    #[test]
    fn test_clouduseraccountsreadonly() {
        assert_eq!("https://www.googleapis.com/auth/cloud.useraccounts.readonly", Scope::CloudUseraccountsReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/cloud.useraccounts.readonly".parse::<Scope>().unwrap(), Scope::CloudUseraccountsReadonly);
    }
    #[test]
    fn test_cloudruntimeconfig() {
        assert_eq!("https://www.googleapis.com/auth/cloudruntimeconfig", Scope::Cloudruntimeconfig.url());
        assert_eq!("https://www.googleapis.com/auth/cloudruntimeconfig".parse::<Scope>().unwrap(), Scope::Cloudruntimeconfig);
    }
    #[test]
    fn test_compute() {
        assert_eq!("https://www.googleapis.com/auth/compute", Scope::Compute.url());
        assert_eq!("https://www.googleapis.com/auth/compute".parse::<Scope>().unwrap(), Scope::Compute);
    }
    #[test]
    fn test_computereadonly() {
        assert_eq!("https://www.googleapis.com/auth/compute.readonly", Scope::ComputeReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/compute.readonly".parse::<Scope>().unwrap(), Scope::ComputeReadonly);
    }
    #[test]
    fn test_contacts() {
        assert_eq!("https://www.googleapis.com/auth/contacts", Scope::Contacts.url());
        assert_eq!("https://www.googleapis.com/auth/contacts".parse::<Scope>().unwrap(), Scope::Contacts);
    }
    #[test]
    fn test_contactsreadonly() {
        assert_eq!("https://www.googleapis.com/auth/contacts.readonly", Scope::ContactsReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/contacts.readonly".parse::<Scope>().unwrap(), Scope::ContactsReadonly);
    }
    #[test]
    fn test_content() {
        assert_eq!("https://www.googleapis.com/auth/content", Scope::Content.url());
        assert_eq!("https://www.googleapis.com/auth/content".parse::<Scope>().unwrap(), Scope::Content);
    }
    #[test]
    fn test_datastore() {
        assert_eq!("https://www.googleapis.com/auth/datastore", Scope::Datastore.url());
        assert_eq!("https://www.googleapis.com/auth/datastore".parse::<Scope>().unwrap(), Scope::Datastore);
    }
    #[test]
    fn test_ddmconversions() {
        assert_eq!("https://www.googleapis.com/auth/ddmconversions", Scope::Ddmconversions.url());
        assert_eq!("https://www.googleapis.com/auth/ddmconversions".parse::<Scope>().unwrap(), Scope::Ddmconversions);
    }
    #[test]
    fn test_devstoragefullcontrol() {
        assert_eq!("https://www.googleapis.com/auth/devstorage.full_control", Scope::DevstorageFullControl.url());
        assert_eq!("https://www.googleapis.com/auth/devstorage.full_control".parse::<Scope>().unwrap(), Scope::DevstorageFullControl);
    }
    #[test]
    fn test_devstoragereadonly() {
        assert_eq!("https://www.googleapis.com/auth/devstorage.read_only", Scope::DevstorageReadOnly.url());
        assert_eq!("https://www.googleapis.com/auth/devstorage.read_only".parse::<Scope>().unwrap(), Scope::DevstorageReadOnly);
    }
    #[test]
    fn test_devstoragereadwrite() {
        assert_eq!("https://www.googleapis.com/auth/devstorage.read_write", Scope::DevstorageReadWrite.url());
        assert_eq!("https://www.googleapis.com/auth/devstorage.read_write".parse::<Scope>().unwrap(), Scope::DevstorageReadWrite);
    }
    #[test]
    fn test_dfareporting() {
        assert_eq!("https://www.googleapis.com/auth/dfareporting", Scope::Dfareporting.url());
        assert_eq!("https://www.googleapis.com/auth/dfareporting".parse::<Scope>().unwrap(), Scope::Dfareporting);
    }
    #[test]
    fn test_dfatrafficking() {
        assert_eq!("https://www.googleapis.com/auth/dfatrafficking", Scope::Dfatrafficking.url());
        assert_eq!("https://www.googleapis.com/auth/dfatrafficking".parse::<Scope>().unwrap(), Scope::Dfatrafficking);
    }
    #[test]
    fn test_doubleclickbidmanager() {
        assert_eq!("https://www.googleapis.com/auth/doubleclickbidmanager", Scope::Doubleclickbidmanager.url());
        assert_eq!("https://www.googleapis.com/auth/doubleclickbidmanager".parse::<Scope>().unwrap(), Scope::Doubleclickbidmanager);
    }
    #[test]
    fn test_doubleclicksearch() {
        assert_eq!("https://www.googleapis.com/auth/doubleclicksearch", Scope::Doubleclicksearch.url());
        assert_eq!("https://www.googleapis.com/auth/doubleclicksearch".parse::<Scope>().unwrap(), Scope::Doubleclicksearch);
    }
    #[test]
    fn test_drive() {
        assert_eq!("https://www.googleapis.com/auth/drive", Scope::Drive.url());
        assert_eq!("https://www.googleapis.com/auth/drive".parse::<Scope>().unwrap(), Scope::Drive);
    }
    #[test]
    fn test_driveappdata() {
        assert_eq!("https://www.googleapis.com/auth/drive.appdata", Scope::DriveAppdata.url());
        assert_eq!("https://www.googleapis.com/auth/drive.appdata".parse::<Scope>().unwrap(), Scope::DriveAppdata);
    }
    #[test]
    fn test_drivefile() {
        assert_eq!("https://www.googleapis.com/auth/drive.file", Scope::DriveFile.url());
        assert_eq!("https://www.googleapis.com/auth/drive.file".parse::<Scope>().unwrap(), Scope::DriveFile);
    }
    #[test]
    fn test_drivemetadata() {
        assert_eq!("https://www.googleapis.com/auth/drive.metadata", Scope::DriveMetadata.url());
        assert_eq!("https://www.googleapis.com/auth/drive.metadata".parse::<Scope>().unwrap(), Scope::DriveMetadata);
    }
    #[test]
    fn test_drivemetadatareadonly() {
        assert_eq!("https://www.googleapis.com/auth/drive.metadata.readonly", Scope::DriveMetadataReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/drive.metadata.readonly".parse::<Scope>().unwrap(), Scope::DriveMetadataReadonly);
    }
    #[test]
    fn test_drivephotosreadonly() {
        assert_eq!("https://www.googleapis.com/auth/drive.photos.readonly", Scope::DrivePhotosReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/drive.photos.readonly".parse::<Scope>().unwrap(), Scope::DrivePhotosReadonly);
    }
    #[test]
    fn test_drivereadonly() {
        assert_eq!("https://www.googleapis.com/auth/drive.readonly", Scope::DriveReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/drive.readonly".parse::<Scope>().unwrap(), Scope::DriveReadonly);
    }
    #[test]
    fn test_drivescripts() {
        assert_eq!("https://www.googleapis.com/auth/drive.scripts", Scope::DriveScripts.url());
        assert_eq!("https://www.googleapis.com/auth/drive.scripts".parse::<Scope>().unwrap(), Scope::DriveScripts);
    }
    #[test]
    fn test_firebase() {
        assert_eq!("https://www.googleapis.com/auth/firebase", Scope::Firebase.url());
        assert_eq!("https://www.googleapis.com/auth/firebase".parse::<Scope>().unwrap(), Scope::Firebase);
    }
    #[test]
    fn test_firebasereadonly() {
        assert_eq!("https://www.googleapis.com/auth/firebase.readonly", Scope::FirebaseReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/firebase.readonly".parse::<Scope>().unwrap(), Scope::FirebaseReadonly);
    }
    #[test]
    fn test_fitnessactivityread() {
        assert_eq!("https://www.googleapis.com/auth/fitness.activity.read", Scope::FitnessActivityRead.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.activity.read".parse::<Scope>().unwrap(), Scope::FitnessActivityRead);
    }
    #[test]
    fn test_fitnessactivitywrite() {
        assert_eq!("https://www.googleapis.com/auth/fitness.activity.write", Scope::FitnessActivityWrite.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.activity.write".parse::<Scope>().unwrap(), Scope::FitnessActivityWrite);
    }
    #[test]
    fn test_fitnessbloodglucoseread() {
        assert_eq!("https://www.googleapis.com/auth/fitness.blood_glucose.read", Scope::FitnessBloodGlucoseRead.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.blood_glucose.read".parse::<Scope>().unwrap(), Scope::FitnessBloodGlucoseRead);
    }
    #[test]
    fn test_fitnessbloodglucosewrite() {
        assert_eq!("https://www.googleapis.com/auth/fitness.blood_glucose.write", Scope::FitnessBloodGlucoseWrite.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.blood_glucose.write".parse::<Scope>().unwrap(), Scope::FitnessBloodGlucoseWrite);
    }
    #[test]
    fn test_fitnessbloodpressureread() {
        assert_eq!("https://www.googleapis.com/auth/fitness.blood_pressure.read", Scope::FitnessBloodPressureRead.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.blood_pressure.read".parse::<Scope>().unwrap(), Scope::FitnessBloodPressureRead);
    }
    #[test]
    fn test_fitnessbloodpressurewrite() {
        assert_eq!("https://www.googleapis.com/auth/fitness.blood_pressure.write", Scope::FitnessBloodPressureWrite.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.blood_pressure.write".parse::<Scope>().unwrap(), Scope::FitnessBloodPressureWrite);
    }
    #[test]
    fn test_fitnessbodyread() {
        assert_eq!("https://www.googleapis.com/auth/fitness.body.read", Scope::FitnessBodyRead.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.body.read".parse::<Scope>().unwrap(), Scope::FitnessBodyRead);
    }
    #[test]
    fn test_fitnessbodytemperatureread() {
        assert_eq!("https://www.googleapis.com/auth/fitness.body_temperature.read", Scope::FitnessBodyTemperatureRead.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.body_temperature.read".parse::<Scope>().unwrap(), Scope::FitnessBodyTemperatureRead);
    }
    #[test]
    fn test_fitnessbodytemperaturewrite() {
        assert_eq!("https://www.googleapis.com/auth/fitness.body_temperature.write", Scope::FitnessBodyTemperatureWrite.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.body_temperature.write".parse::<Scope>().unwrap(), Scope::FitnessBodyTemperatureWrite);
    }
    #[test]
    fn test_fitnessbodywrite() {
        assert_eq!("https://www.googleapis.com/auth/fitness.body.write", Scope::FitnessBodyWrite.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.body.write".parse::<Scope>().unwrap(), Scope::FitnessBodyWrite);
    }
    #[test]
    fn test_fitnesslocationread() {
        assert_eq!("https://www.googleapis.com/auth/fitness.location.read", Scope::FitnessLocationRead.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.location.read".parse::<Scope>().unwrap(), Scope::FitnessLocationRead);
    }
    #[test]
    fn test_fitnesslocationwrite() {
        assert_eq!("https://www.googleapis.com/auth/fitness.location.write", Scope::FitnessLocationWrite.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.location.write".parse::<Scope>().unwrap(), Scope::FitnessLocationWrite);
    }
    #[test]
    fn test_fitnessnutritionread() {
        assert_eq!("https://www.googleapis.com/auth/fitness.nutrition.read", Scope::FitnessNutritionRead.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.nutrition.read".parse::<Scope>().unwrap(), Scope::FitnessNutritionRead);
    }
    #[test]
    fn test_fitnessnutritionwrite() {
        assert_eq!("https://www.googleapis.com/auth/fitness.nutrition.write", Scope::FitnessNutritionWrite.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.nutrition.write".parse::<Scope>().unwrap(), Scope::FitnessNutritionWrite);
    }
    #[test]
    fn test_fitnessoxygensaturationread() {
        assert_eq!("https://www.googleapis.com/auth/fitness.oxygen_saturation.read", Scope::FitnessOxygenSaturationRead.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.oxygen_saturation.read".parse::<Scope>().unwrap(), Scope::FitnessOxygenSaturationRead);
    }
    #[test]
    fn test_fitnessoxygensaturationwrite() {
        assert_eq!("https://www.googleapis.com/auth/fitness.oxygen_saturation.write", Scope::FitnessOxygenSaturationWrite.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.oxygen_saturation.write".parse::<Scope>().unwrap(), Scope::FitnessOxygenSaturationWrite);
    }
    #[test]
    fn test_fitnessreproductivehealthread() {
        assert_eq!("https://www.googleapis.com/auth/fitness.reproductive_health.read", Scope::FitnessReproductiveHealthRead.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.reproductive_health.read".parse::<Scope>().unwrap(), Scope::FitnessReproductiveHealthRead);
    }
    #[test]
    fn test_fitnessreproductivehealthwrite() {
        assert_eq!("https://www.googleapis.com/auth/fitness.reproductive_health.write", Scope::FitnessReproductiveHealthWrite.url());
        assert_eq!("https://www.googleapis.com/auth/fitness.reproductive_health.write".parse::<Scope>().unwrap(), Scope::FitnessReproductiveHealthWrite);
    }
    #[test]
    fn test_forms() {
        assert_eq!("https://www.googleapis.com/auth/forms", Scope::Forms.url());
        assert_eq!("https://www.googleapis.com/auth/forms".parse::<Scope>().unwrap(), Scope::Forms);
    }
    #[test]
    fn test_formscurrentonly() {
        assert_eq!("https://www.googleapis.com/auth/forms.currentonly", Scope::FormsCurrentonly.url());
        assert_eq!("https://www.googleapis.com/auth/forms.currentonly".parse::<Scope>().unwrap(), Scope::FormsCurrentonly);
    }
    #[test]
    fn test_fusiontables() {
        assert_eq!("https://www.googleapis.com/auth/fusiontables", Scope::Fusiontables.url());
        assert_eq!("https://www.googleapis.com/auth/fusiontables".parse::<Scope>().unwrap(), Scope::Fusiontables);
    }
    #[test]
    fn test_fusiontablesreadonly() {
        assert_eq!("https://www.googleapis.com/auth/fusiontables.readonly", Scope::FusiontablesReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/fusiontables.readonly".parse::<Scope>().unwrap(), Scope::FusiontablesReadonly);
    }
    #[test]
    fn test_games() {
        assert_eq!("https://www.googleapis.com/auth/games", Scope::Games.url());
        assert_eq!("https://www.googleapis.com/auth/games".parse::<Scope>().unwrap(), Scope::Games);
    }
    #[test]
    fn test_genomics() {
        assert_eq!("https://www.googleapis.com/auth/genomics", Scope::Genomics.url());
        assert_eq!("https://www.googleapis.com/auth/genomics".parse::<Scope>().unwrap(), Scope::Genomics);
    }
    #[test]
    fn test_genomicsreadonly() {
        assert_eq!("https://www.googleapis.com/auth/genomics.readonly", Scope::GenomicsReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/genomics.readonly".parse::<Scope>().unwrap(), Scope::GenomicsReadonly);
    }
    #[test]
    fn test_glasslocation() {
        assert_eq!("https://www.googleapis.com/auth/glass.location", Scope::GlassLocation.url());
        assert_eq!("https://www.googleapis.com/auth/glass.location".parse::<Scope>().unwrap(), Scope::GlassLocation);
    }
    #[test]
    fn test_glasstimeline() {
        assert_eq!("https://www.googleapis.com/auth/glass.timeline", Scope::GlassTimeline.url());
        assert_eq!("https://www.googleapis.com/auth/glass.timeline".parse::<Scope>().unwrap(), Scope::GlassTimeline);
    }
    #[test]
    fn test_gmail() {
        assert_eq!("https://mail.google.com/", Scope::Gmail.url());
        assert_eq!("https://mail.google.com/".parse::<Scope>().unwrap(), Scope::Gmail);
    }
    #[test]
    fn test_gmailcompose() {
        assert_eq!("https://www.googleapis.com/auth/gmail.compose", Scope::GmailCompose.url());
        assert_eq!("https://www.googleapis.com/auth/gmail.compose".parse::<Scope>().unwrap(), Scope::GmailCompose);
    }
    #[test]
    fn test_gmailinsert() {
        assert_eq!("https://www.googleapis.com/auth/gmail.insert", Scope::GmailInsert.url());
        assert_eq!("https://www.googleapis.com/auth/gmail.insert".parse::<Scope>().unwrap(), Scope::GmailInsert);
    }
    #[test]
    fn test_gmaillabels() {
        assert_eq!("https://www.googleapis.com/auth/gmail.labels", Scope::GmailLabels.url());
        assert_eq!("https://www.googleapis.com/auth/gmail.labels".parse::<Scope>().unwrap(), Scope::GmailLabels);
    }
    #[test]
    fn test_gmailmetadata() {
        assert_eq!("https://www.googleapis.com/auth/gmail.metadata", Scope::GmailMetadata.url());
        assert_eq!("https://www.googleapis.com/auth/gmail.metadata".parse::<Scope>().unwrap(), Scope::GmailMetadata);
    }
    #[test]
    fn test_gmailmodify() {
        assert_eq!("https://www.googleapis.com/auth/gmail.modify", Scope::GmailModify.url());
        assert_eq!("https://www.googleapis.com/auth/gmail.modify".parse::<Scope>().unwrap(), Scope::GmailModify);
    }
    #[test]
    fn test_gmailreadonly() {
        assert_eq!("https://www.googleapis.com/auth/gmail.readonly", Scope::GmailReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/gmail.readonly".parse::<Scope>().unwrap(), Scope::GmailReadonly);
    }
    #[test]
    fn test_gmailsend() {
        assert_eq!("https://www.googleapis.com/auth/gmail.send", Scope::GmailSend.url());
        assert_eq!("https://www.googleapis.com/auth/gmail.send".parse::<Scope>().unwrap(), Scope::GmailSend);
    }
    #[test]
    fn test_gmailsettingsbasic() {
        assert_eq!("https://www.googleapis.com/auth/gmail.settings.basic", Scope::GmailSettingsBasic.url());
        assert_eq!("https://www.googleapis.com/auth/gmail.settings.basic".parse::<Scope>().unwrap(), Scope::GmailSettingsBasic);
    }
    #[test]
    fn test_gmailsettingssharing() {
        assert_eq!("https://www.googleapis.com/auth/gmail.settings.sharing", Scope::GmailSettingsSharing.url());
        assert_eq!("https://www.googleapis.com/auth/gmail.settings.sharing".parse::<Scope>().unwrap(), Scope::GmailSettingsSharing);
    }
    #[test]
    fn test_groups() {
        assert_eq!("https://www.googleapis.com/auth/groups", Scope::Groups.url());
        assert_eq!("https://www.googleapis.com/auth/groups".parse::<Scope>().unwrap(), Scope::Groups);
    }
    #[test]
    fn test_loggingadmin() {
        assert_eq!("https://www.googleapis.com/auth/logging.admin", Scope::LoggingAdmin.url());
        assert_eq!("https://www.googleapis.com/auth/logging.admin".parse::<Scope>().unwrap(), Scope::LoggingAdmin);
    }
    #[test]
    fn test_loggingread() {
        assert_eq!("https://www.googleapis.com/auth/logging.read", Scope::LoggingRead.url());
        assert_eq!("https://www.googleapis.com/auth/logging.read".parse::<Scope>().unwrap(), Scope::LoggingRead);
    }
    #[test]
    fn test_loggingwrite() {
        assert_eq!("https://www.googleapis.com/auth/logging.write", Scope::LoggingWrite.url());
        assert_eq!("https://www.googleapis.com/auth/logging.write".parse::<Scope>().unwrap(), Scope::LoggingWrite);
    }
    #[test]
    fn test_m8feeds() {
        assert_eq!("https://www.google.com/m8/feeds", Scope::M8Feeds.url());
        assert_eq!("https://www.google.com/m8/feeds".parse::<Scope>().unwrap(), Scope::M8Feeds);
    }
    #[test]
    fn test_manufacturercenter() {
        assert_eq!("https://www.googleapis.com/auth/manufacturercenter", Scope::Manufacturercenter.url());
        assert_eq!("https://www.googleapis.com/auth/manufacturercenter".parse::<Scope>().unwrap(), Scope::Manufacturercenter);
    }
    #[test]
    fn test_monitoring() {
        assert_eq!("https://www.googleapis.com/auth/monitoring", Scope::Monitoring.url());
        assert_eq!("https://www.googleapis.com/auth/monitoring".parse::<Scope>().unwrap(), Scope::Monitoring);
    }
    #[test]
    fn test_monitoringread() {
        assert_eq!("https://www.googleapis.com/auth/monitoring.read", Scope::MonitoringRead.url());
        assert_eq!("https://www.googleapis.com/auth/monitoring.read".parse::<Scope>().unwrap(), Scope::MonitoringRead);
    }
    #[test]
    fn test_monitoringwrite() {
        assert_eq!("https://www.googleapis.com/auth/monitoring.write", Scope::MonitoringWrite.url());
        assert_eq!("https://www.googleapis.com/auth/monitoring.write".parse::<Scope>().unwrap(), Scope::MonitoringWrite);
    }
    #[test]
    fn test_ndevclouddnsreadonly() {
        assert_eq!("https://www.googleapis.com/auth/ndev.clouddns.readonly", Scope::NdevClouddnsReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/ndev.clouddns.readonly".parse::<Scope>().unwrap(), Scope::NdevClouddnsReadonly);
    }
    #[test]
    fn test_ndevclouddnsreadwrite() {
        assert_eq!("https://www.googleapis.com/auth/ndev.clouddns.readwrite", Scope::NdevClouddnsReadwrite.url());
        assert_eq!("https://www.googleapis.com/auth/ndev.clouddns.readwrite".parse::<Scope>().unwrap(), Scope::NdevClouddnsReadwrite);
    }
    #[test]
    fn test_ndevcloudman() {
        assert_eq!("https://www.googleapis.com/auth/ndev.cloudman", Scope::NdevCloudman.url());
        assert_eq!("https://www.googleapis.com/auth/ndev.cloudman".parse::<Scope>().unwrap(), Scope::NdevCloudman);
    }
    #[test]
    fn test_ndevcloudmanreadonly() {
        assert_eq!("https://www.googleapis.com/auth/ndev.cloudman.readonly", Scope::NdevCloudmanReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/ndev.cloudman.readonly".parse::<Scope>().unwrap(), Scope::NdevCloudmanReadonly);
    }
    #[test]
    fn test_playmoviespartnerreadonly() {
        assert_eq!("https://www.googleapis.com/auth/playmovies_partner.readonly", Scope::PlaymoviesPartnerReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/playmovies_partner.readonly".parse::<Scope>().unwrap(), Scope::PlaymoviesPartnerReadonly);
    }
    #[test]
    fn test_pluscirclesread() {
        assert_eq!("https://www.googleapis.com/auth/plus.circles.read", Scope::PlusCirclesRead.url());
        assert_eq!("https://www.googleapis.com/auth/plus.circles.read".parse::<Scope>().unwrap(), Scope::PlusCirclesRead);
    }
    #[test]
    fn test_pluscircleswrite() {
        assert_eq!("https://www.googleapis.com/auth/plus.circles.write", Scope::PlusCirclesWrite.url());
        assert_eq!("https://www.googleapis.com/auth/plus.circles.write".parse::<Scope>().unwrap(), Scope::PlusCirclesWrite);
    }
    #[test]
    fn test_pluslogin() {
        assert_eq!("https://www.googleapis.com/auth/plus.login", Scope::PlusLogin.url());
        assert_eq!("https://www.googleapis.com/auth/plus.login".parse::<Scope>().unwrap(), Scope::PlusLogin);
    }
    #[test]
    fn test_plusme() {
        assert_eq!("https://www.googleapis.com/auth/plus.me", Scope::PlusMe.url());
        assert_eq!("https://www.googleapis.com/auth/plus.me".parse::<Scope>().unwrap(), Scope::PlusMe);
    }
    #[test]
    fn test_plusmediaupload() {
        assert_eq!("https://www.googleapis.com/auth/plus.media.upload", Scope::PlusMediaUpload.url());
        assert_eq!("https://www.googleapis.com/auth/plus.media.upload".parse::<Scope>().unwrap(), Scope::PlusMediaUpload);
    }
    #[test]
    fn test_plusprofilesread() {
        assert_eq!("https://www.googleapis.com/auth/plus.profiles.read", Scope::PlusProfilesRead.url());
        assert_eq!("https://www.googleapis.com/auth/plus.profiles.read".parse::<Scope>().unwrap(), Scope::PlusProfilesRead);
    }
    #[test]
    fn test_plusstreamread() {
        assert_eq!("https://www.googleapis.com/auth/plus.stream.read", Scope::PlusStreamRead.url());
        assert_eq!("https://www.googleapis.com/auth/plus.stream.read".parse::<Scope>().unwrap(), Scope::PlusStreamRead);
    }
    #[test]
    fn test_plusstreamwrite() {
        assert_eq!("https://www.googleapis.com/auth/plus.stream.write", Scope::PlusStreamWrite.url());
        assert_eq!("https://www.googleapis.com/auth/plus.stream.write".parse::<Scope>().unwrap(), Scope::PlusStreamWrite);
    }
    #[test]
    fn test_prediction() {
        assert_eq!("https://www.googleapis.com/auth/prediction", Scope::Prediction.url());
        assert_eq!("https://www.googleapis.com/auth/prediction".parse::<Scope>().unwrap(), Scope::Prediction);
    }
    #[test]
    fn test_presentations() {
        assert_eq!("https://www.googleapis.com/auth/presentations", Scope::Presentations.url());
        assert_eq!("https://www.googleapis.com/auth/presentations".parse::<Scope>().unwrap(), Scope::Presentations);
    }
    #[test]
    fn test_presentationsreadonly() {
        assert_eq!("https://www.googleapis.com/auth/presentations.readonly", Scope::PresentationsReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/presentations.readonly".parse::<Scope>().unwrap(), Scope::PresentationsReadonly);
    }
    #[test]
    fn test_pubsub() {
        assert_eq!("https://www.googleapis.com/auth/pubsub", Scope::Pubsub.url());
        assert_eq!("https://www.googleapis.com/auth/pubsub".parse::<Scope>().unwrap(), Scope::Pubsub);
    }
    #[test]
    fn test_replicapool() {
        assert_eq!("https://www.googleapis.com/auth/replicapool", Scope::Replicapool.url());
        assert_eq!("https://www.googleapis.com/auth/replicapool".parse::<Scope>().unwrap(), Scope::Replicapool);
    }
    #[test]
    fn test_replicapoolreadonly() {
        assert_eq!("https://www.googleapis.com/auth/replicapool.readonly", Scope::ReplicapoolReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/replicapool.readonly".parse::<Scope>().unwrap(), Scope::ReplicapoolReadonly);
    }
    #[test]
    fn test_servicemanagement() {
        assert_eq!("https://www.googleapis.com/auth/service.management", Scope::ServiceManagement.url());
        assert_eq!("https://www.googleapis.com/auth/service.management".parse::<Scope>().unwrap(), Scope::ServiceManagement);
    }
    #[test]
    fn test_servicemanagementreadonly() {
        assert_eq!("https://www.googleapis.com/auth/service.management.readonly", Scope::ServiceManagementReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/service.management.readonly".parse::<Scope>().unwrap(), Scope::ServiceManagementReadonly);
    }
    #[test]
    fn test_servicecontrol() {
        assert_eq!("https://www.googleapis.com/auth/servicecontrol", Scope::Servicecontrol.url());
        assert_eq!("https://www.googleapis.com/auth/servicecontrol".parse::<Scope>().unwrap(), Scope::Servicecontrol);
    }
    #[test]
    fn test_siteverification() {
        assert_eq!("https://www.googleapis.com/auth/siteverification", Scope::Siteverification.url());
        assert_eq!("https://www.googleapis.com/auth/siteverification".parse::<Scope>().unwrap(), Scope::Siteverification);
    }
    #[test]
    fn test_siteverificationverifyonly() {
        assert_eq!("https://www.googleapis.com/auth/siteverification.verify_only", Scope::SiteverificationVerifyOnly.url());
        assert_eq!("https://www.googleapis.com/auth/siteverification.verify_only".parse::<Scope>().unwrap(), Scope::SiteverificationVerifyOnly);
    }
    #[test]
    fn test_sourcereadonly() {
        assert_eq!("https://www.googleapis.com/auth/source.read_only", Scope::SourceReadOnly.url());
        assert_eq!("https://www.googleapis.com/auth/source.read_only".parse::<Scope>().unwrap(), Scope::SourceReadOnly);
    }
    #[test]
    fn test_sourcereadwrite() {
        assert_eq!("https://www.googleapis.com/auth/source.read_write", Scope::SourceReadWrite.url());
        assert_eq!("https://www.googleapis.com/auth/source.read_write".parse::<Scope>().unwrap(), Scope::SourceReadWrite);
    }
    #[test]
    fn test_spanneradmin() {
        assert_eq!("https://www.googleapis.com/auth/spanner.admin", Scope::SpannerAdmin.url());
        assert_eq!("https://www.googleapis.com/auth/spanner.admin".parse::<Scope>().unwrap(), Scope::SpannerAdmin);
    }
    #[test]
    fn test_spannerdata() {
        assert_eq!("https://www.googleapis.com/auth/spanner.data", Scope::SpannerData.url());
        assert_eq!("https://www.googleapis.com/auth/spanner.data".parse::<Scope>().unwrap(), Scope::SpannerData);
    }
    #[test]
    fn test_spreadsheets() {
        assert_eq!("https://www.googleapis.com/auth/spreadsheets", Scope::Spreadsheets.url());
        assert_eq!("https://www.googleapis.com/auth/spreadsheets".parse::<Scope>().unwrap(), Scope::Spreadsheets);
    }
    #[test]
    fn test_spreadsheetsreadonly() {
        assert_eq!("https://www.googleapis.com/auth/spreadsheets.readonly", Scope::SpreadsheetsReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/spreadsheets.readonly".parse::<Scope>().unwrap(), Scope::SpreadsheetsReadonly);
    }
    #[test]
    fn test_sqlserviceadmin() {
        assert_eq!("https://www.googleapis.com/auth/sqlservice.admin", Scope::SqlserviceAdmin.url());
        assert_eq!("https://www.googleapis.com/auth/sqlservice.admin".parse::<Scope>().unwrap(), Scope::SqlserviceAdmin);
    }
    #[test]
    fn test_tagmanagerdeletecontainers() {
        assert_eq!("https://www.googleapis.com/auth/tagmanager.delete.containers", Scope::TagmanagerDeleteContainers.url());
        assert_eq!("https://www.googleapis.com/auth/tagmanager.delete.containers".parse::<Scope>().unwrap(), Scope::TagmanagerDeleteContainers);
    }
    #[test]
    fn test_tagmanagereditcontainers() {
        assert_eq!("https://www.googleapis.com/auth/tagmanager.edit.containers", Scope::TagmanagerEditContainers.url());
        assert_eq!("https://www.googleapis.com/auth/tagmanager.edit.containers".parse::<Scope>().unwrap(), Scope::TagmanagerEditContainers);
    }
    #[test]
    fn test_tagmanagereditcontainerversions() {
        assert_eq!("https://www.googleapis.com/auth/tagmanager.edit.containerversions", Scope::TagmanagerEditContainerversions.url());
        assert_eq!("https://www.googleapis.com/auth/tagmanager.edit.containerversions".parse::<Scope>().unwrap(), Scope::TagmanagerEditContainerversions);
    }
    #[test]
    fn test_tagmanagermanageaccounts() {
        assert_eq!("https://www.googleapis.com/auth/tagmanager.manage.accounts", Scope::TagmanagerManageAccounts.url());
        assert_eq!("https://www.googleapis.com/auth/tagmanager.manage.accounts".parse::<Scope>().unwrap(), Scope::TagmanagerManageAccounts);
    }
    #[test]
    fn test_tagmanagermanageusers() {
        assert_eq!("https://www.googleapis.com/auth/tagmanager.manage.users", Scope::TagmanagerManageUsers.url());
        assert_eq!("https://www.googleapis.com/auth/tagmanager.manage.users".parse::<Scope>().unwrap(), Scope::TagmanagerManageUsers);
    }
    #[test]
    fn test_tagmanagerpublish() {
        assert_eq!("https://www.googleapis.com/auth/tagmanager.publish", Scope::TagmanagerPublish.url());
        assert_eq!("https://www.googleapis.com/auth/tagmanager.publish".parse::<Scope>().unwrap(), Scope::TagmanagerPublish);
    }
    #[test]
    fn test_tagmanagerreadonly() {
        assert_eq!("https://www.googleapis.com/auth/tagmanager.readonly", Scope::TagmanagerReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/tagmanager.readonly".parse::<Scope>().unwrap(), Scope::TagmanagerReadonly);
    }
    #[test]
    fn test_taskqueue() {
        assert_eq!("https://www.googleapis.com/auth/taskqueue", Scope::Taskqueue.url());
        assert_eq!("https://www.googleapis.com/auth/taskqueue".parse::<Scope>().unwrap(), Scope::Taskqueue);
    }
    #[test]
    fn test_taskqueueconsumer() {
        assert_eq!("https://www.googleapis.com/auth/taskqueue.consumer", Scope::TaskqueueConsumer.url());
        assert_eq!("https://www.googleapis.com/auth/taskqueue.consumer".parse::<Scope>().unwrap(), Scope::TaskqueueConsumer);
    }
    #[test]
    fn test_tasks() {
        assert_eq!("https://www.googleapis.com/auth/tasks", Scope::Tasks.url());
        assert_eq!("https://www.googleapis.com/auth/tasks".parse::<Scope>().unwrap(), Scope::Tasks);
    }
    #[test]
    fn test_tasksreadonly() {
        assert_eq!("https://www.googleapis.com/auth/tasks.readonly", Scope::TasksReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/tasks.readonly".parse::<Scope>().unwrap(), Scope::TasksReadonly);
    }
    #[test]
    fn test_urlshortener() {
        assert_eq!("https://www.googleapis.com/auth/urlshortener", Scope::Urlshortener.url());
        assert_eq!("https://www.googleapis.com/auth/urlshortener".parse::<Scope>().unwrap(), Scope::Urlshortener);
    }
    #[test]
    fn test_useraddressesread() {
        assert_eq!("https://www.googleapis.com/auth/user.addresses.read", Scope::UserAddressesRead.url());
        assert_eq!("https://www.googleapis.com/auth/user.addresses.read".parse::<Scope>().unwrap(), Scope::UserAddressesRead);
    }
    #[test]
    fn test_userbirthdayread() {
        assert_eq!("https://www.googleapis.com/auth/user.birthday.read", Scope::UserBirthdayRead.url());
        assert_eq!("https://www.googleapis.com/auth/user.birthday.read".parse::<Scope>().unwrap(), Scope::UserBirthdayRead);
    }
    #[test]
    fn test_useremailsread() {
        assert_eq!("https://www.googleapis.com/auth/user.emails.read", Scope::UserEmailsRead.url());
        assert_eq!("https://www.googleapis.com/auth/user.emails.read".parse::<Scope>().unwrap(), Scope::UserEmailsRead);
    }
    #[test]
    fn test_userphonenumbersread() {
        assert_eq!("https://www.googleapis.com/auth/user.phonenumbers.read", Scope::UserPhonenumbersRead.url());
        assert_eq!("https://www.googleapis.com/auth/user.phonenumbers.read".parse::<Scope>().unwrap(), Scope::UserPhonenumbersRead);
    }
    #[test]
    fn test_userinfoemail() {
        assert_eq!("https://www.googleapis.com/auth/userinfo.email", Scope::UserinfoEmail.url());
        assert_eq!("https://www.googleapis.com/auth/userinfo.email".parse::<Scope>().unwrap(), Scope::UserinfoEmail);
    }
    #[test]
    fn test_userinfoprofile() {
        assert_eq!("https://www.googleapis.com/auth/userinfo.profile", Scope::UserinfoProfile.url());
        assert_eq!("https://www.googleapis.com/auth/userinfo.profile".parse::<Scope>().unwrap(), Scope::UserinfoProfile);
    }
    #[test]
    fn test_userlocationbeaconregistry() {
        assert_eq!("https://www.googleapis.com/auth/userlocation.beacon.registry", Scope::UserlocationBeaconRegistry.url());
        assert_eq!("https://www.googleapis.com/auth/userlocation.beacon.registry".parse::<Scope>().unwrap(), Scope::UserlocationBeaconRegistry);
    }
    #[test]
    fn test_webmasters() {
        assert_eq!("https://www.googleapis.com/auth/webmasters", Scope::Webmasters.url());
        assert_eq!("https://www.googleapis.com/auth/webmasters".parse::<Scope>().unwrap(), Scope::Webmasters);
    }
    #[test]
    fn test_webmastersreadonly() {
        assert_eq!("https://www.googleapis.com/auth/webmasters.readonly", Scope::WebmastersReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/webmasters.readonly".parse::<Scope>().unwrap(), Scope::WebmastersReadonly);
    }
    #[test]
    fn test_xapizoo() {
        assert_eq!("https://www.googleapis.com/auth/xapi.zoo", Scope::XapiZoo.url());
        assert_eq!("https://www.googleapis.com/auth/xapi.zoo".parse::<Scope>().unwrap(), Scope::XapiZoo);
    }
    #[test]
    fn test_youtube() {
        assert_eq!("https://www.googleapis.com/auth/youtube", Scope::Youtube.url());
        assert_eq!("https://www.googleapis.com/auth/youtube".parse::<Scope>().unwrap(), Scope::Youtube);
    }
    #[test]
    fn test_youtubeforcessl() {
        assert_eq!("https://www.googleapis.com/auth/youtube.force-ssl", Scope::YoutubeForceSsl.url());
        assert_eq!("https://www.googleapis.com/auth/youtube.force-ssl".parse::<Scope>().unwrap(), Scope::YoutubeForceSsl);
    }
    #[test]
    fn test_youtubereadonly() {
        assert_eq!("https://www.googleapis.com/auth/youtube.readonly", Scope::YoutubeReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/youtube.readonly".parse::<Scope>().unwrap(), Scope::YoutubeReadonly);
    }
    #[test]
    fn test_youtubeupload() {
        assert_eq!("https://www.googleapis.com/auth/youtube.upload", Scope::YoutubeUpload.url());
        assert_eq!("https://www.googleapis.com/auth/youtube.upload".parse::<Scope>().unwrap(), Scope::YoutubeUpload);
    }
    #[test]
    fn test_youtubepartner() {
        assert_eq!("https://www.googleapis.com/auth/youtubepartner", Scope::Youtubepartner.url());
        assert_eq!("https://www.googleapis.com/auth/youtubepartner".parse::<Scope>().unwrap(), Scope::Youtubepartner);
    }
    #[test]
    fn test_youtubepartnerchannelaudit() {
        assert_eq!("https://www.googleapis.com/auth/youtubepartner-channel-audit", Scope::YoutubepartnerChannelAudit.url());
        assert_eq!("https://www.googleapis.com/auth/youtubepartner-channel-audit".parse::<Scope>().unwrap(), Scope::YoutubepartnerChannelAudit);
    }
    #[test]
    fn test_ytanalyticsmonetaryreadonly() {
        assert_eq!("https://www.googleapis.com/auth/yt-analytics-monetary.readonly", Scope::YtAnalyticsMonetaryReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/yt-analytics-monetary.readonly".parse::<Scope>().unwrap(), Scope::YtAnalyticsMonetaryReadonly);
    }
    #[test]
    fn test_ytanalyticsreadonly() {
        assert_eq!("https://www.googleapis.com/auth/yt-analytics.readonly", Scope::YtAnalyticsReadonly.url());
        assert_eq!("https://www.googleapis.com/auth/yt-analytics.readonly".parse::<Scope>().unwrap(), Scope::YtAnalyticsReadonly);
    }
}