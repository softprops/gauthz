use std::ascii::AsciiExt;
use std::collections::HashMap;

const SCRAPED: &str = "https://www.googleapis.com/auth/adexchange.buyer
https://www.googleapis.com/auth/adexchange.seller
https://www.googleapis.com/auth/adexchange.seller.readonly
https://www.googleapis.com/auth/xapi.zoo
https://www.googleapis.com/auth/admin.datatransfer
https://www.googleapis.com/auth/admin.datatransfer.readonly
https://www.googleapis.com/auth/admin.directory.customer
https://www.googleapis.com/auth/admin.directory.customer.readonly
https://www.googleapis.com/auth/admin.directory.device.chromeos
https://www.googleapis.com/auth/admin.directory.device.chromeos.readonly
https://www.googleapis.com/auth/admin.directory.device.mobile
https://www.googleapis.com/auth/admin.directory.device.mobile.action
https://www.googleapis.com/auth/admin.directory.device.mobile.readonly
https://www.googleapis.com/auth/admin.directory.domain
https://www.googleapis.com/auth/admin.directory.domain.readonly
https://www.googleapis.com/auth/admin.directory.group
https://www.googleapis.com/auth/admin.directory.group.member
https://www.googleapis.com/auth/admin.directory.group.member.readonly
https://www.googleapis.com/auth/admin.directory.group.readonly
https://www.googleapis.com/auth/admin.directory.notifications
https://www.googleapis.com/auth/admin.directory.orgunit
https://www.googleapis.com/auth/admin.directory.orgunit.readonly
https://www.googleapis.com/auth/admin.directory.resource.calendar
https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly
https://www.googleapis.com/auth/admin.directory.rolemanagement
https://www.googleapis.com/auth/admin.directory.rolemanagement.readonly
https://www.googleapis.com/auth/admin.directory.user
https://www.googleapis.com/auth/admin.directory.user.alias
https://www.googleapis.com/auth/admin.directory.user.alias.readonly
https://www.googleapis.com/auth/admin.directory.user.readonly
https://www.googleapis.com/auth/admin.directory.user.security
https://www.googleapis.com/auth/admin.directory.userschema
https://www.googleapis.com/auth/admin.directory.userschema.readonly
https://www.googleapis.com/auth/admin.reports.audit.readonly
https://www.googleapis.com/auth/admin.reports.usage.readonly
https://www.googleapis.com/auth/adsense
https://www.googleapis.com/auth/adsense.readonly
https://www.googleapis.com/auth/adsensehost
https://www.googleapis.com/auth/analytics
https://www.googleapis.com/auth/analytics.edit
https://www.googleapis.com/auth/analytics.manage.users
https://www.googleapis.com/auth/analytics.manage.users.readonly
https://www.googleapis.com/auth/analytics.provision
https://www.googleapis.com/auth/analytics.readonly
https://www.googleapis.com/auth/analytics
https://www.googleapis.com/auth/analytics.readonly
https://www.googleapis.com/auth/androidenterprise
https://www.googleapis.com/auth/androidpublisher
https://www.googleapis.com/auth/appengine.admin
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/activity
https://www.googleapis.com/auth/drive
https://www.googleapis.com/auth/drive.metadata
https://www.googleapis.com/auth/drive.metadata.readonly
https://www.googleapis.com/auth/drive.readonly
https://www.googleapis.com/auth/appstate
https://www.googleapis.com/auth/bigquery
https://www.googleapis.com/auth/bigquery.insertdata
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/devstorage.full_control
https://www.googleapis.com/auth/devstorage.read_only
https://www.googleapis.com/auth/devstorage.read_write
https://www.googleapis.com/auth/bigquery
https://www.googleapis.com/auth/blogger
https://www.googleapis.com/auth/blogger.readonly
https://www.googleapis.com/auth/books
https://www.googleapis.com/auth/calendar
https://www.googleapis.com/auth/calendar.readonly
https://www.googleapis.com/auth/classroom.courses
https://www.googleapis.com/auth/classroom.courses.readonly
https://www.googleapis.com/auth/classroom.coursework.me
https://www.googleapis.com/auth/classroom.coursework.me.readonly
https://www.googleapis.com/auth/classroom.coursework.students
https://www.googleapis.com/auth/classroom.coursework.students.readonly
https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly
https://www.googleapis.com/auth/classroom.guardianlinks.students
https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly
https://www.googleapis.com/auth/classroom.profile.emails
https://www.googleapis.com/auth/classroom.profile.photos
https://www.googleapis.com/auth/classroom.rosters
https://www.googleapis.com/auth/classroom.rosters.readonly
https://www.googleapis.com/auth/classroom.student-submissions.me.readonly
https://www.googleapis.com/auth/classroom.student-submissions.students.readonly
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud_debugger
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/monitoring
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/cloud.useraccounts
https://www.googleapis.com/auth/cloud.useraccounts.readonly
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/compute
https://www.googleapis.com/auth/compute.readonly
https://www.googleapis.com/auth/devstorage.full_control
https://www.googleapis.com/auth/devstorage.read_only
https://www.googleapis.com/auth/devstorage.read_write
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/content
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/compute
https://www.googleapis.com/auth/compute.readonly
https://www.googleapis.com/auth/userinfo.email
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/datastore
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/ndev.cloudman
https://www.googleapis.com/auth/ndev.cloudman.readonly
https://www.googleapis.com/auth/ddmconversions
https://www.googleapis.com/auth/dfareporting
https://www.googleapis.com/auth/dfatrafficking
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/ndev.clouddns.readonly
https://www.googleapis.com/auth/ndev.clouddns.readwrite
https://www.googleapis.com/auth/doubleclickbidmanager
https://www.googleapis.com/auth/doubleclicksearch
https://www.googleapis.com/auth/drive
https://www.googleapis.com/auth/drive.appdata
https://www.googleapis.com/auth/drive.file
https://www.googleapis.com/auth/drive.metadata
https://www.googleapis.com/auth/drive.metadata.readonly
https://www.googleapis.com/auth/drive.photos.readonly
https://www.googleapis.com/auth/drive.readonly
https://www.googleapis.com/auth/drive.scripts
https://www.googleapis.com/auth/firebase
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/firebase
https://www.googleapis.com/auth/firebase.readonly
https://www.googleapis.com/auth/fitness.activity.read
https://www.googleapis.com/auth/fitness.activity.write
https://www.googleapis.com/auth/fitness.blood_glucose.read
https://www.googleapis.com/auth/fitness.blood_glucose.write
https://www.googleapis.com/auth/fitness.blood_pressure.read
https://www.googleapis.com/auth/fitness.blood_pressure.write
https://www.googleapis.com/auth/fitness.body.read
https://www.googleapis.com/auth/fitness.body.write
https://www.googleapis.com/auth/fitness.body_temperature.read
https://www.googleapis.com/auth/fitness.body_temperature.write
https://www.googleapis.com/auth/fitness.location.read
https://www.googleapis.com/auth/fitness.location.write
https://www.googleapis.com/auth/fitness.nutrition.read
https://www.googleapis.com/auth/fitness.nutrition.write
https://www.googleapis.com/auth/fitness.oxygen_saturation.read
https://www.googleapis.com/auth/fitness.oxygen_saturation.write
https://www.googleapis.com/auth/fitness.reproductive_health.read
https://www.googleapis.com/auth/fitness.reproductive_health.write
https://www.googleapis.com/auth/fusiontables
https://www.googleapis.com/auth/fusiontables.readonly
https://www.googleapis.com/auth/drive.appdata
https://www.googleapis.com/auth/games
https://www.googleapis.com/auth/plus.login
https://www.googleapis.com/auth/androidpublisher
https://www.googleapis.com/auth/games
https://www.googleapis.com/auth/plus.login
https://www.googleapis.com/auth/bigquery
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/devstorage.read_write
https://www.googleapis.com/auth/genomics
https://www.googleapis.com/auth/genomics.readonly
https://mail.google.com/
https://www.googleapis.com/auth/gmail.compose
https://www.googleapis.com/auth/gmail.insert
https://www.googleapis.com/auth/gmail.labels
https://www.googleapis.com/auth/gmail.metadata
https://www.googleapis.com/auth/gmail.modify
https://www.googleapis.com/auth/gmail.readonly
https://www.googleapis.com/auth/gmail.send
https://www.googleapis.com/auth/gmail.settings.basic
https://www.googleapis.com/auth/gmail.settings.sharing
https://www.googleapis.com/auth/apps.groups.migration
https://www.googleapis.com/auth/apps.groups.settings
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/firebase
https://www.googleapis.com/auth/cloud-language
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/apps.licensing
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/logging.admin
https://www.googleapis.com/auth/logging.read
https://www.googleapis.com/auth/logging.write
https://www.googleapis.com/auth/manufacturercenter
https://www.googleapis.com/auth/glass.location
https://www.googleapis.com/auth/glass.timeline
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/monitoring
https://www.googleapis.com/auth/monitoring.read
https://www.googleapis.com/auth/monitoring.write
https://www.googleapis.com/auth/plus.login
https://www.googleapis.com/auth/plus.me
https://www.googleapis.com/auth/userinfo.email
https://www.googleapis.com/auth/userinfo.profile
https://www.googleapis.com/auth/contacts
https://www.googleapis.com/auth/contacts.readonly
https://www.googleapis.com/auth/plus.login
https://www.googleapis.com/auth/user.addresses.read
https://www.googleapis.com/auth/user.birthday.read
https://www.googleapis.com/auth/user.emails.read
https://www.googleapis.com/auth/user.phonenumbers.read
https://www.googleapis.com/auth/userinfo.email
https://www.googleapis.com/auth/userinfo.profile
https://www.googleapis.com/auth/androidpublisher
https://www.googleapis.com/auth/playmovies_partner.readonly
https://www.googleapis.com/auth/plus.login
https://www.googleapis.com/auth/plus.me
https://www.googleapis.com/auth/userinfo.email
https://www.googleapis.com/auth/userinfo.profile
https://www.googleapis.com/auth/plus.circles.read
https://www.googleapis.com/auth/plus.circles.write
https://www.googleapis.com/auth/plus.login
https://www.googleapis.com/auth/plus.me
https://www.googleapis.com/auth/plus.media.upload
https://www.googleapis.com/auth/plus.profiles.read
https://www.googleapis.com/auth/plus.stream.read
https://www.googleapis.com/auth/plus.stream.write
https://www.googleapis.com/auth/userinfo.email
https://www.googleapis.com/auth/userinfo.profile
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/devstorage.full_control
https://www.googleapis.com/auth/devstorage.read_only
https://www.googleapis.com/auth/devstorage.read_write
https://www.googleapis.com/auth/prediction
https://www.googleapis.com/auth/userlocation.beacon.registry
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/pubsub
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/compute
https://www.googleapis.com/auth/compute.readonly
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/replicapool
https://www.googleapis.com/auth/replicapool.readonly
https://www.googleapis.com/auth/apps.order
https://www.googleapis.com/auth/apps.order.readonly
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/compute
https://www.googleapis.com/auth/compute.readonly
https://www.googleapis.com/auth/ndev.cloudman
https://www.googleapis.com/auth/ndev.cloudman.readonly
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloudruntimeconfig
https://mail.google.com/
https://www.google.com/calendar/feeds
https://www.google.com/m8/feeds
https://www.googleapis.com/auth/admin.directory.group
https://www.googleapis.com/auth/admin.directory.user
https://www.googleapis.com/auth/drive
https://www.googleapis.com/auth/forms
https://www.googleapis.com/auth/forms.currentonly
https://www.googleapis.com/auth/groups
https://www.googleapis.com/auth/spreadsheets
https://www.googleapis.com/auth/userinfo.email
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/servicecontrol
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/service.management
https://www.googleapis.com/auth/service.management.readonly
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/service.management
https://www.googleapis.com/auth/drive
https://www.googleapis.com/auth/drive.file
https://www.googleapis.com/auth/drive.readonly
https://www.googleapis.com/auth/spreadsheets
https://www.googleapis.com/auth/spreadsheets.readonly
https://www.googleapis.com/auth/siteverification
https://www.googleapis.com/auth/siteverification.verify_only
https://www.googleapis.com/auth/drive
https://www.googleapis.com/auth/drive.readonly
https://www.googleapis.com/auth/presentations
https://www.googleapis.com/auth/presentations.readonly
https://www.googleapis.com/auth/spreadsheets
https://www.googleapis.com/auth/spreadsheets.readonly
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/source.read_only
https://www.googleapis.com/auth/source.read_write
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/spanner.admin
https://www.googleapis.com/auth/spanner.data
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/sqlservice.admin
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-platform.read-only
https://www.googleapis.com/auth/devstorage.full_control
https://www.googleapis.com/auth/devstorage.read_only
https://www.googleapis.com/auth/devstorage.read_write
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/tagmanager.delete.containers
https://www.googleapis.com/auth/tagmanager.edit.containers
https://www.googleapis.com/auth/tagmanager.edit.containerversions
https://www.googleapis.com/auth/tagmanager.manage.accounts
https://www.googleapis.com/auth/tagmanager.manage.users
https://www.googleapis.com/auth/tagmanager.publish
https://www.googleapis.com/auth/tagmanager.readonly
https://www.googleapis.com/auth/taskqueue
https://www.googleapis.com/auth/taskqueue.consumer
https://www.googleapis.com/auth/tasks
https://www.googleapis.com/auth/tasks.readonly
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/cloud-translation
https://www.googleapis.com/auth/urlshortener
https://www.googleapis.com/auth/cloud-platform
https://www.googleapis.com/auth/webmasters
https://www.googleapis.com/auth/webmasters.readonly
https://www.googleapis.com/auth/youtube
https://www.googleapis.com/auth/youtube.force-ssl
https://www.googleapis.com/auth/youtube.readonly
https://www.googleapis.com/auth/youtube.upload
https://www.googleapis.com/auth/youtubepartner
https://www.googleapis.com/auth/youtubepartner-channel-audit
https://www.googleapis.com/auth/youtube
https://www.googleapis.com/auth/youtube.readonly
https://www.googleapis.com/auth/youtubepartner
https://www.googleapis.com/auth/yt-analytics-monetary.readonly
https://www.googleapis.com/auth/yt-analytics.readonly
https://www.googleapis.com/auth/yt-analytics-monetary.readonly
https://www.googleapis.com/auth/yt-analytics.readonly";

fn nameify(scope: &str, url: &str) -> String {
    // handle google mail case
    if scope == "" {
        return "Gmail".to_owned();
    }
    let (name, _) = scope.chars().fold((String::new(), true), |result, c| {
        let (mut name, up) = result;
        if up {
            name.push(c.to_ascii_uppercase());
            (name, false)
        } else if c == '.' || c == '-' || c == '_' {
            (name, true)
        } else {
            name.push(c);
            (name, false)
        }
    });
    // handle odd feeds cases
    if "Feeds" == name {
        if url.contains("calendar") {
            return "CalendarFeeds".to_owned();
        } else {
            return "M8Feeds".to_owned();
        }
    }
    name
}

fn main() {
    let mappings = SCRAPED.lines().fold(HashMap::new(), |mut result, url| {
        let mut components = url.split("/").collect::<Vec<_>>();
        components.reverse();
        for scope in components.get(0) {
            result.insert(nameify(scope, &url), url);
        }
        result
    });
    let mut sorted = mappings.iter().collect::<Vec<_>>();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    println!("pub enum Scope {{");
    for &(k, v) in sorted.iter() {
        println!("    /// Scope for {}", v);
        println!("    {},", k);
    }
    println!("}}");

    println!("impl Scope {{");
    println!("    /// Return url assocaited with scope");
    println!("    pub fn url(&self) -> &'static str {{");
    println!("        match *self {{");
    for &(k, v) in sorted.iter() {
        println!("             Scope::{} => \"{}\",", k, v);
    }
    println!("        }}");
    println!("    }}");
    println!("    /// Return Scope assocaited with url");
    println!("    pub fn scope(url: &str) -> Option<Scope> {{");
    println!("        match url {{");
    for &(k, v) in sorted.iter() {
        println!("             \"{}\" => Some(Scope::{}),", v, k);
    }
    println!("                 _ => None");
    println!("        }}");
    println!("    }}");
    println!("}}");
}
